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

//! Utilities for transferring heap-allocated Rust `Vec<T>` values across an FFI boundary.
//! 用于跨 FFI 边界传输堆分配的 Rust `Vec<T>` 值的工具。
//!
//! The primary abstraction offered by this module is `CVec`, a C-compatible struct that stores
//! a raw pointer (`ptr`) together with the vector's logical `len` and `cap`.  By moving the
//! allocation metadata into a plain `repr(C)` type we allow the memory created by Rust to be
//! owned, inspected, and ultimately freed by foreign code (or vice-versa) without introducing
//! undefined behaviour.
//! 此模块提供的主要抽象是 `CVec`，一个 C 兼容的结构体，存储原始指针（`ptr`）以及向量的逻辑 `len` 和 `cap`。
//! 通过将分配元数据移动到普通的 `repr(C)` 类型中，我们允许由 Rust 创建的内存被外部代码拥有、检查和最终释放（反之亦然），而不会引入未定义行为。
//!
//! Only a very small API surface is exposed to C:
//! 只有非常小的 API 表面暴露给 C：
//!
//! * `cvec_new` – create an empty `CVec` sentinel that can be returned to foreign code.
//!   `cvec_new` – 创建一个可以返回给外部代码的空 `CVec` 哨兵。
//!
//! De-allocation is intentionally **not** provided via a generic helper. Instead each FFI module
//! must expose its own *type-specific* `vec_*_drop` function which reconstructs the original
//! `Vec<T>` with [`Vec::from_raw_parts`] and allows it to drop. This avoids the size-mismatch risk
//! that a one-size-fits-all `cvec_drop` had in the past.
//! 故意 **不** 通过通用辅助函数提供释放。相反，每个 FFI 模块必须公开自己的 *类型特定* `vec_*_drop` 函数，
//! 该函数使用 [`Vec::from_raw_parts`] 重建原始 `Vec<T>` 并允许其释放。这避免了过去通用 `cvec_drop` 的大小不匹配风险。
//!
//! All other manipulation happens on the Rust side before relinquishing ownership.  This keeps the
//! rules for memory safety straightforward: foreign callers must treat the memory region pointed
//! to by `ptr` as **opaque** and interact with it solely through the functions provided here.
//! 所有其他操作在放弃所有权之前在 Rust 端进行。这使内存安全规则简单明了：
//! 外部调用者必须将 `ptr` 指向的内存区域视为 **不透明**，并仅通过此处提供的函数与其交互。

use std::{ffi::c_void, fmt::Display, ptr::null};

use crate::ffi::abort_on_panic;

/// `CVec` is a C compatible struct that stores an opaque pointer to a block of
/// memory, its length and the capacity of the vector it was allocated from.
/// `CVec` 是一个 C 兼容的结构体，存储指向内存块的不透明指针、其长度以及分配它的向量的容量。
///
/// # Safety
/// # 安全性
///
/// Changing the values here may lead to undefined behavior when the memory is dropped.
/// 更改此处的值可能在释放内存时导致未定义行为。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CVec {
    /// Opaque pointer to block of memory storing elements to access the
    /// elements cast it to the underlying type.
    /// 指向存储元素的内存块的不透明指针，要访问元素需要将其转换为底层类型。
    pub ptr: *mut c_void,
    /// The number of elements in the block.
    /// 块中的元素数量。
    pub len: usize,
    /// The capacity of vector from which it was allocated.
    /// Used when deallocating the memory
    /// 分配它的向量的容量。在释放内存时使用。
    pub cap: usize,
}

// SAFETY: CVec is marked as Send to satisfy PyO3's PyCapsule requirements, which need
// to transfer ownership across the Python/Rust boundary. However, CVec contains raw
// pointers and is only safe to use in single-threaded contexts or with external
// synchronization guarantees.
// 安全性：CVec 被标记为 Send 以满足 PyO3 的 PyCapsule 要求，需要跨 Python/Rust 边界传输所有权。
// 但是，CVec 包含原始指针，仅在单线程上下文中或具有外部同步保证时使用才是安全的。
//
// The Send impl is required for:
// Send impl 需要用于：
// 1. PyO3's PyCapsule::new_with_destructor which has a Send bound
//    PyO3 的 PyCapsule::new_with_destructor，它具有 Send 约束
// 2. Transferring CVec ownership to Python (which runs on a single GIL-protected thread)
//    将 CVec 所有权传输到 Python（在单个 GIL 保护的线程上运行）
//
// IMPORTANT: Do not send CVec instances across threads without ensuring:
// 重要：不要在没有确保以下条件的情况下跨线程发送 CVec 实例：
// - The underlying data type T is itself Send + Sync
//   底层数据类型 T 本身是 Send + Sync
// - Proper external synchronization (e.g., mutex) protects concurrent access
//   适当的外部同步（例如，互斥锁）保护并发访问
// - The CVec is consumed on the same thread where it will be reconstructed
//   CVec 在将重建它的同一线程上被消费
//
// In practice, CVec usage in this codebase is confined to the Python FFI boundary
// where the Python GIL provides the necessary synchronization.
// 实际上，此代码库中的 CVec 使用仅限于 Python FFI 边界，其中 Python GIL 提供必要的同步。
unsafe impl Send for CVec {}

impl CVec {
    /// Returns an empty [`CVec`].
    /// 返回一个空的 [`CVec`]。
    ///
    /// This is primarily useful for constructing a sentinel value that represents the
    /// absence of data when crossing the FFI boundary.
    /// 这对于构造一个哨兵值很有用，该值表示跨 FFI 边界时没有数据。
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            // Explicitly type cast the pointer to some type to satisfy the
            // compiler. Since the pointer is null it works for any type.
            // 显式将指针类型转换为某种类型以满足编译器。由于指针为空，它适用于任何类型。
            ptr: null::<bool>() as *mut c_void,
            len: 0,
            cap: 0,
        }
    }
}

/// Consumes and leaks the Vec, returning a mutable pointer to the contents as
/// a [`CVec`]. The memory has been leaked and now exists for the lifetime of the
/// program unless dropped manually.
/// Note: drop the memory by reconstructing the vec using `from_raw_parts` method
/// as shown in the test below.
/// 消费并泄漏 Vec，将指向内容的可变指针作为 [`CVec`] 返回。
/// 内存已被泄漏，现在存在于程序的整个生命周期中，除非手动释放。
/// 注意：通过使用 `from_raw_parts` 方法重建 vec 来释放内存，如下面的测试所示。
impl<T> From<Vec<T>> for CVec {
    fn from(mut data: Vec<T>) -> Self {
        if data.is_empty() {
            Self::empty()
        } else {
            let len = data.len();
            let cap = data.capacity();
            let ptr = data.as_mut_ptr();
            std::mem::forget(data);
            Self {
                ptr: ptr.cast::<std::ffi::c_void>(),
                len,
                cap,
            }
        }
    }
}

impl Display for CVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CVec {{ ptr: {:?}, len: {}, cap: {} }}",
            self.ptr, self.len, self.cap,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// C API
////////////////////////////////////////////////////////////////////////////////

/// Construct a new *empty* [`CVec`] value for use as initialiser/sentinel in foreign code.
/// 构造一个新的 *空* [`CVec`] 值，用作外部代码中的初始化器/哨兵。
#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub extern "C" fn cvec_new() -> CVec {
    abort_on_panic(CVec::empty)
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::CVec;

    /// Access values from a vector converted into a [`CVec`].
    #[rstest]
    #[allow(unused_assignments)]
    fn access_values_test() {
        let test_data = vec![1_u64, 2, 3];
        let mut vec_len = 0;
        let mut vec_cap = 0;
        let cvec: CVec = {
            let data = test_data.clone();
            vec_len = data.len();
            vec_cap = data.capacity();
            data.into()
        };

        let CVec { ptr, len, cap } = cvec;
        assert_eq!(len, vec_len);
        assert_eq!(cap, vec_cap);

        let data = ptr.cast::<u64>();
        unsafe {
            assert_eq!(*data, test_data[0]);
            assert_eq!(*data.add(1), test_data[1]);
            assert_eq!(*data.add(2), test_data[2]);
        }

        unsafe {
            // reconstruct the struct and drop the memory to deallocate
            let _ = Vec::from_raw_parts(ptr.cast::<u64>(), len, cap);
        }
    }

    /// An empty vector gets converted to a null pointer wrapped in a [`CVec`].
    #[rstest]
    fn empty_vec_should_give_null_ptr() {
        let data: Vec<u64> = vec![];
        let cvec: CVec = data.into();
        assert_eq!(cvec.ptr.cast::<u64>(), std::ptr::null_mut::<u64>());
    }
}
