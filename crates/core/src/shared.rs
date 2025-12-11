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

//! Efficient and ergonomic wrappers around frequently-used `Rc<RefCell<T>>` / `Weak<RefCell<T>>` pairs.
//! 围绕常用的 `Rc<RefCell<T>>` / `Weak<RefCell<T>>` 对的高效且符合人体工程学的包装器。
//!
//! The NautilusTrader codebase heavily relies on shared, interior-mutable ownership for many
//! engine components (`Rc<RefCell<T>>`). Repeating that verbose type across many APIs—alongside
//! its weak counterpart—clutters code and increases the likelihood of accidentally storing a
//! strong reference where only a weak reference is required (leading to reference cycles).
//! NautilusTrader 代码库严重依赖共享的内部可变所有权用于许多引擎组件（`Rc<RefCell<T>>`）。
//! 在许多 API 中重复这种冗长的类型及其弱对应项会使代码混乱，并增加在只需要弱引用的地方意外存储强引用的可能性（导致引用循环）。
//!
//! `SharedCell<T>` and `WeakCell<T>` are zero-cost new-types that make the intent explicit and
//! offer convenience helpers (`downgrade`, `upgrade`, `borrow`, `borrow_mut`). Because the
//! wrappers are `#[repr(transparent)]`, they have the exact same memory layout as the wrapped
//! `Rc` / `Weak` and introduce no runtime overhead.
//! `SharedCell<T>` 和 `WeakCell<T>` 是零成本新类型，使意图明确并提供便利的辅助函数（`downgrade`、`upgrade`、`borrow`、`borrow_mut`）。
//! 由于包装器是 `#[repr(transparent)]`，它们与包装的 `Rc` / `Weak` 具有完全相同的内存布局，并且不引入运行时开销。

//! ## Choosing between `SharedCell` and `WeakCell`
//! ## 在 `SharedCell` 和 `WeakCell` 之间选择
//!
//! * Use **`SharedCell<T>`** when the current owner genuinely *owns* (or co-owns) the value –
//!   just as you would normally store an `Rc<RefCell<T>>`.
//!   当当前所有者真正 *拥有*（或共同拥有）该值时，使用 **`SharedCell<T>`**——就像您通常存储 `Rc<RefCell<T>>` 一样。
//! * Use **`WeakCell<T>`** for back-references that could otherwise form a reference cycle.
//!   The back-pointer does **not** keep the value alive, and every access must first
//!   `upgrade()` to a strong `SharedCell`. This pattern is how we break circular ownership such
//!   as *Exchange ↔ `ExecutionClient`*: the exchange keeps a `SharedCell` to the client, while the
//!   client holds only a `WeakCell` back to the exchange.
//!   对于可能形成引用循环的反向引用，使用 **`WeakCell<T>`**。
//!   反向指针 **不会** 保持值存活，每次访问必须首先 `upgrade()` 为强 `SharedCell`。
//!   这种模式是我们打破循环所有权的方式，例如 *Exchange ↔ `ExecutionClient`*：交易所保持对客户端的 `SharedCell`，而客户端仅持有对交易所的 `WeakCell`。

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

/// Strong, shared ownership of `T` with interior mutability.
/// 具有内部可变性的 `T` 的强共享所有权。
#[repr(transparent)]
#[derive(Debug)]
pub struct SharedCell<T>(Rc<RefCell<T>>);

impl<T> Clone for SharedCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SharedCell<T> {
    /// Wraps a value inside `Rc<RefCell<..>>`.
    /// 将值包装在 `Rc<RefCell<..>>` 内。
    #[inline]
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    /// Creates a [`WeakCell`] pointing to the same allocation.
    /// 创建一个指向相同分配的 [`WeakCell`]。
    #[inline]
    #[must_use]
    pub fn downgrade(&self) -> WeakCell<T> {
        WeakCell(Rc::downgrade(&self.0))
    }

    /// Immutable borrow of the inner value.
    /// 内部值的不可变借用。
    #[inline]
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    /// Mutable borrow of the inner value.
    /// 内部值的可变借用。
    #[inline]
    #[must_use]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    /// Number of active strong references.
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    /// Number of active weak references.
    #[inline]
    #[must_use]
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.0)
    }
}

impl<T> From<Rc<RefCell<T>>> for SharedCell<T> {
    fn from(inner: Rc<RefCell<T>>) -> Self {
        Self(inner)
    }
}

impl<T> From<SharedCell<T>> for Rc<RefCell<T>> {
    fn from(shared: SharedCell<T>) -> Self {
        shared.0
    }
}

impl<T> std::ops::Deref for SharedCell<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Weak counterpart to [`SharedCell`].
#[repr(transparent)]
#[derive(Debug)]
pub struct WeakCell<T>(Weak<RefCell<T>>);

impl<T> Clone for WeakCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> WeakCell<T> {
    /// Attempts to upgrade the weak reference to a strong [`SharedCell`].
    #[inline]
    pub fn upgrade(&self) -> Option<SharedCell<T>> {
        self.0.upgrade().map(SharedCell)
    }

    /// Returns `true` if the pointed-to value has been dropped.
    #[inline]
    #[must_use]
    pub fn is_dropped(&self) -> bool {
        self.0.strong_count() == 0
    }
}

impl<T> From<Weak<RefCell<T>>> for WeakCell<T> {
    fn from(inner: Weak<RefCell<T>>) -> Self {
        Self(inner)
    }
}

impl<T> From<WeakCell<T>> for Weak<RefCell<T>> {
    fn from(cell: WeakCell<T>) -> Self {
        cell.0
    }
}
