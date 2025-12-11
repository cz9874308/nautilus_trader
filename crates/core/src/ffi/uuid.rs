// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! FFI helpers for the [`UUID4`] wrapper type.
//! [`UUID4`] 包装类型的 FFI 辅助函数。
//!
//! The functions exported here make it possible for C/Python code to create, compare, and hash
//! UUID values *without* having to understand the internal representation chosen by
//! NautilusTrader.
//! 此处导出的函数使 C/Python 代码可以创建、比较和哈希 UUID 值，*而无需* 理解 NautilusTrader 选择的内部表示。

use std::{
    collections::hash_map::DefaultHasher,
    ffi::{CStr, c_char},
    hash::{Hash, Hasher},
};

use crate::{UUID4, ffi::abort_on_panic};

/// Generate a new random (version-4) UUID and return it by value.
/// 生成一个新的随机（版本 4）UUID 并按值返回。
#[unsafe(no_mangle)]
pub extern "C" fn uuid4_new() -> UUID4 {
    abort_on_panic(UUID4::new)
}

/// Returns a [`UUID4`] from C string pointer.
/// 从 C 字符串指针返回 [`UUID4`]。
///
/// # Safety
/// # 安全性
///
/// Assumes `ptr` is a valid C string pointer.
/// 假设 `ptr` 是有效的 C 字符串指针。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if `ptr` cannot be cast to a valid C string.
/// 如果 `ptr` 无法转换为有效的 C 字符串，则会 panic。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uuid4_from_cstr(ptr: *const c_char) -> UUID4 {
    abort_on_panic(|| {
        assert!(!ptr.is_null(), "`ptr` was NULL");
        // SAFETY: Caller guarantees ptr is valid per function contract
        // 安全性：调用者根据函数契约保证 ptr 有效
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let value = cstr.to_str().expect("Failed to convert C string to UTF-8");
        UUID4::from(value)
    })
}

/// Return a borrowed *null-terminated* UTF-8 C string representing `uuid`.
/// 返回表示 `uuid` 的借用的 *空终止* UTF-8 C 字符串。
///
/// The pointer remains valid for as long as the input `UUID4` reference lives – callers **must
/// not** attempt to free it.
/// 只要输入 `UUID4` 引用存在，指针就保持有效——调用者 **不得** 尝试释放它。
#[unsafe(no_mangle)]
pub extern "C" fn uuid4_to_cstr(uuid: &UUID4) -> *const c_char {
    abort_on_panic(|| uuid.to_cstr().as_ptr())
}

/// Compare two UUID values, returning `1` when they are equal and `0` otherwise.
/// 比较两个 UUID 值，相等时返回 `1`，否则返回 `0`。
#[unsafe(no_mangle)]
pub extern "C" fn uuid4_eq(lhs: &UUID4, rhs: &UUID4) -> u8 {
    abort_on_panic(|| u8::from(lhs == rhs))
}

/// Compute the stable [`u64`] hash of `uuid` using Rust's default hasher.
/// 使用 Rust 的默认哈希器计算 `uuid` 的稳定 [`u64`] 哈希值。
#[unsafe(no_mangle)]
pub extern "C" fn uuid4_hash(uuid: &UUID4) -> u64 {
    abort_on_panic(|| {
        let mut h = DefaultHasher::new();
        uuid.hash(&mut h);
        h.finish()
    })
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use rstest::*;
    use uuid::{self, Uuid};

    use super::*;

    #[rstest]
    fn test_new() {
        let uuid = uuid4_new();
        let uuid_string = uuid.to_string();
        let uuid_parsed = Uuid::parse_str(&uuid_string).expect("Uuid::parse_str failed");
        assert_eq!(uuid_parsed.get_version().unwrap(), uuid::Version::Random);
    }

    #[rstest]
    fn test_from_cstr() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid_cstring = CString::new(uuid_string).expect("CString::new failed");
        let uuid_ptr = uuid_cstring.as_ptr();
        let uuid = unsafe { uuid4_from_cstr(uuid_ptr) };
        assert_eq!(uuid_string, uuid.to_string());
    }

    #[rstest]
    fn test_to_cstr() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);
        let uuid_ptr = uuid4_to_cstr(&uuid);
        let uuid_cstr = unsafe { CStr::from_ptr(uuid_ptr) };
        let uuid_result_string = uuid_cstr.to_str().expect("CStr::to_str failed").to_string();
        assert_eq!(uuid_string, uuid_result_string);
    }

    #[rstest]
    fn test_eq() {
        let uuid1 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c757");
        let uuid2 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c757");
        let uuid3 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c758");
        assert_eq!(uuid4_eq(&uuid1, &uuid2), 1);
        assert_eq!(uuid4_eq(&uuid1, &uuid3), 0);
    }

    #[rstest]
    fn test_hash() {
        let uuid1 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c757");
        let uuid2 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c757");
        let uuid3 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c758");
        assert_eq!(uuid4_hash(&uuid1), uuid4_hash(&uuid2));
        assert_ne!(uuid4_hash(&uuid1), uuid4_hash(&uuid3));
    }
}
