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

//! C foreign function interface (FFI) from [cbindgen](https://github.com/mozilla/cbindgen).
//! 来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口（FFI）。
//!
//! All exported functions route through `abort_on_panic` so that any panic inside the
//! Rust implementation aborts immediately instead of unwinding across the foreign boundary.
//! Unwinding into C/Python is undefined behaviour, so this keeps the existing fail-fast
//! semantics while avoiding subtle stack corruption during debugging.
//! 所有导出的函数都通过 `abort_on_panic` 路由，以便 Rust 实现内部的任何 panic 立即中止，
//! 而不是跨外部边界展开。展开到 C/Python 是未定义行为，因此这保持了现有的快速失败语义，
//! 同时避免了调试期间的微妙堆栈损坏。

#![allow(unsafe_code)]
#![allow(unsafe_attr_outside_unsafe)]

pub mod cvec;
pub mod datetime;
pub mod parsing;
pub mod string;
pub mod uuid;

use std::{
    panic::{self, AssertUnwindSafe},
    process,
};

/// Executes `f`, aborting the process if it panics.
/// 执行 `f`，如果发生 panic 则中止进程。
///
/// FFI exports always call this helper so a panic never unwinds across the
/// `extern "C"` boundary. Unwinding into C/Python is undefined behaviour and
/// can silently corrupt the foreign stack; aborting instead preserves the
/// fail-fast guarantee with effectively no debugging downside (the panic
/// message is still logged before the abort).
/// FFI 导出总是调用此辅助函数，以便 panic 永远不会跨 `extern "C"` 边界展开。
/// 展开到 C/Python 是未定义行为，可能静默损坏外部堆栈；中止反而保留了快速失败保证，
/// 实际上没有调试缺点（panic 消息仍在中止前记录）。
#[inline]
pub(crate) fn abort_on_panic<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(_) => process::abort(),
    }
}
