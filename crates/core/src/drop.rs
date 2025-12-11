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

//! Explicit, manually-invocable cleanup hook used to break reference cycles before `Drop`.
//! 用于在 `Drop` 之前打破引用循环的显式、可手动调用的清理钩子。
//!
//! Many long-lived components register callbacks or handlers that retain strong references back to
//! them, creating reference-count cycles that prevent Rust's automatic destructor (`Drop`) from
//! running.  The `CleanDrop` trait provides an *object-safe* method, `clean_drop`, that can be
//! called explicitly (e.g. during an orderly shutdown) to release such resources.  Implementations
//! should also call `clean_drop` from their `Drop` impl as a final safety net.
//! 许多长期存在的组件注册回调或处理程序，这些回调或处理程序保留对它们的强引用，创建引用计数循环，阻止 Rust 的自动析构函数（`Drop`）运行。
//! `CleanDrop` trait 提供一个 *对象安全* 的方法 `clean_drop`，可以显式调用（例如在有序关闭期间）以释放此类资源。
//! 实现还应该在其 `Drop` impl 中调用 `clean_drop` 作为最终的安全网。
//!
//! Design contract:
//! 设计契约：
//! 1. **Idempotent** – multiple calls must be safe.
//!    **幂等性** – 多次调用必须是安全的。
//! 2. Perform all externally-observable cleanup here (unregister handlers, abort tasks, clear
//!    callbacks, downgrade `Rc`/`Arc` references, etc.).
//!    在此处执行所有外部可观察的清理（注销处理程序、中止任务、清除回调、降级 `Rc`/`Arc` 引用等）。

/// Trait providing an explicit cleanup method that may be invoked prior to `Drop`.
/// 提供可在 `Drop` 之前调用的显式清理方法的 trait。
pub trait CleanDrop {
    /// Perform custom cleanup, releasing external resources and breaking strong reference cycles.
    /// 执行自定义清理，释放外部资源并打破强引用循环。
    fn clean_drop(&mut self);
}
