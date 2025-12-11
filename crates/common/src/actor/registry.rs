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

//! Thread-local actor registry with lifetime-safe access guards.
//! 具有生命周期安全访问保护的线程本地参与者注册表。
//!
//! # Design
//! # 设计
//!
//! The actor registry stores actors in thread-local storage and provides access via
//! [`ActorRef<T>`] guards. This design addresses several constraints:
//! 参与者注册表将参与者存储在线程本地存储中，并通过 [`ActorRef<T>`] 保护提供访问。
//! 此设计解决了几个约束：
//!
//! - **Use-after-free prevention**: `ActorRef` holds an `Rc` clone, keeping the actor
//!   alive even if removed from the registry while the guard exists.
//!   **防止释放后使用**：`ActorRef` 持有 `Rc` 克隆，即使参与者在保护存在时从注册表中移除，也能保持参与者存活。
//! - **Re-entrant callbacks**: Message handlers frequently call back into the registry
//!   to access other actors. Unlike `RefCell`-style borrow tracking, multiple `ActorRef`
//!   guards can exist simultaneously without panicking.
//!   **可重入回调**：消息处理器经常回调注册表以访问其他参与者。
//!   与 `RefCell` 样式的借用跟踪不同，多个 `ActorRef` 保护可以同时存在而不会 panic。
//! - **No `'static` lifetime lie**: Previous designs returned `&'static mut T`, which
//!   didn't reflect actual validity. The guard-based approach ties the borrow to the
//!   guard's lifetime.
//!   **没有 `'static` 生命周期谎言**：以前的设计返回 `&'static mut T`，这不能反映实际有效性。
//!   基于保护的方法将借用绑定到保护的生命周期。
//!
//! # Limitations
//! # 限制
//!
//! - **Aliasing not prevented**: Two guards can exist for the same actor simultaneously,
//!   allowing aliased mutable access. This is technically undefined behavior but is
//!   required by the re-entrant callback pattern. Higher-level discipline is required.
//!   **不防止别名**：两个保护可以同时存在于同一参与者，允许别名可变访问。
//!   这在技术上是未定义的行为，但可重入回调模式需要它。需要更高级别的约束。
//! - **Thread-local only**: Guards must not be sent across threads.
//!   **仅线程本地**：保护不得跨线程发送。

use std::{
    any::TypeId,
    cell::{RefCell, UnsafeCell},
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use ahash::AHashMap;
use ustr::Ustr;

use super::Actor;

/// A guard providing mutable access to an actor.
/// 提供对参与者的可变访问的保护。
///
/// This guard holds an `Rc` reference to keep the actor alive, preventing
/// use-after-free if the actor is removed from the registry while the guard
/// exists. The guard implements `Deref` and `DerefMut` for ergonomic access.
/// 此保护持有 `Rc` 引用以保持参与者存活，如果参与者在保护存在时从注册表中移除，
/// 则防止释放后使用。保护实现 `Deref` 和 `DerefMut` 以提供符合人体工程学的访问。
///
/// # Safety
/// # 安全性
///
/// While this guard prevents use-after-free from registry removal, it does not
/// prevent aliasing. Multiple `ActorRef` instances can exist for the same actor
/// simultaneously, which is technically undefined behavior but is required by
/// the re-entrant callback pattern in this codebase.
/// 虽然此保护可以防止因注册表移除而导致的释放后使用，但它不能防止别名。
/// 多个 `ActorRef` 实例可以同时存在于同一参与者，这在技术上是未定义的行为，
/// 但此代码库中的可重入回调模式需要它。
pub struct ActorRef<T: Actor> {
    actor_rc: Rc<UnsafeCell<dyn Actor>>,
    _marker: PhantomData<T>,
}

impl<T: Actor> Debug for ActorRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActorRef")
            .field("actor_id", &self.deref().id())
            .finish()
    }
}

impl<T: Actor> Deref for ActorRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Type was verified at construction time
        unsafe { &*(self.actor_rc.get() as *const T) }
    }
}

impl<T: Actor> DerefMut for ActorRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Type was verified at construction time
        unsafe { &mut *(self.actor_rc.get() as *mut T) }
    }
}

thread_local! {
    static ACTOR_REGISTRY: ActorRegistry = ActorRegistry::new();
}

/// Registry for storing actors.
/// 用于存储参与者的注册表。
pub struct ActorRegistry {
    actors: RefCell<AHashMap<Ustr, Rc<UnsafeCell<dyn Actor>>>>,
}

impl Debug for ActorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actors_ref = self.actors.borrow();
        let keys: Vec<&Ustr> = actors_ref.keys().collect();
        f.debug_struct(stringify!(ActorRegistry))
            .field("actors", &keys)
            .finish()
    }
}

impl Default for ActorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorRegistry {
    /// Creates a new actor registry.
    /// 创建一个新的参与者注册表。
    pub fn new() -> Self {
        Self {
            actors: RefCell::new(AHashMap::new()),
        }
    }

    /// Inserts an actor into the registry.
    /// 将参与者插入注册表。
    pub fn insert(&self, id: Ustr, actor: Rc<UnsafeCell<dyn Actor>>) {
        let mut actors = self.actors.borrow_mut();
        if actors.contains_key(&id) {
            log::warn!("Replacing existing actor with id: {id}");
        }
        actors.insert(id, actor);
    }

    pub fn get(&self, id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
        self.actors.borrow().get(id).cloned()
    }

    /// Returns the number of registered actors.
    pub fn len(&self) -> usize {
        self.actors.borrow().len()
    }

    /// Checks if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.actors.borrow().is_empty()
    }

    /// Removes an actor from the registry.
    pub fn remove(&self, id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
        self.actors.borrow_mut().remove(id)
    }

    /// Checks if an actor with the `id` exists.
    pub fn contains(&self, id: &Ustr) -> bool {
        self.actors.borrow().contains_key(id)
    }
}

pub fn get_actor_registry() -> &'static ActorRegistry {
    ACTOR_REGISTRY.with(|registry| unsafe {
        // SAFETY: We return a static reference that lives for the lifetime of the thread.
        // Since this is thread_local storage, each thread has its own instance.
        // The transmute extends the lifetime to 'static which is safe because
        // thread_local ensures the registry lives for the thread's entire lifetime.
        std::mem::transmute::<&ActorRegistry, &'static ActorRegistry>(registry)
    })
}

/// Registers an actor.
pub fn register_actor<T>(actor: T) -> Rc<UnsafeCell<T>>
where
    T: Actor + 'static,
{
    let actor_id = actor.id();
    let actor_ref = Rc::new(UnsafeCell::new(actor));

    // Register as Actor (message handling only)
    let actor_trait_ref: Rc<UnsafeCell<dyn Actor>> = actor_ref.clone();
    get_actor_registry().insert(actor_id, actor_trait_ref);

    actor_ref
}

pub fn get_actor(id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
    get_actor_registry().get(id)
}

/// Returns a guard providing mutable access to the registered actor of type `T`.
///
/// The returned [`ActorRef`] holds an `Rc` to keep the actor alive, preventing
/// use-after-free if the actor is removed from the registry.
///
/// # Panics
///
/// - Panics if no actor with the specified `id` is found in the registry.
/// - Panics if the stored actor is not of type `T`.
///
/// # Safety
///
/// While this function is not marked `unsafe`, aliasing constraints apply:
///
/// - **Aliasing**: The caller should ensure no other mutable references to the same
///   actor exist simultaneously. The callback-based message handling pattern in this
///   codebase requires re-entrant access, which technically violates this invariant.
/// - **Thread safety**: The registry is thread-local; do not send guards across
///   threads.
#[must_use]
pub fn get_actor_unchecked<T: Actor>(id: &Ustr) -> ActorRef<T> {
    let registry = get_actor_registry();
    let actor_rc = registry
        .get(id)
        .unwrap_or_else(|| panic!("Actor for {id} not found"));

    // SAFETY: Get a reference to check the type before casting
    let actor_ref = unsafe { &*actor_rc.get() };
    let actual_type = actor_ref.as_any().type_id();
    let expected_type = TypeId::of::<T>();

    if actual_type != expected_type {
        panic!("Actor type mismatch for '{id}': expected {expected_type:?}, found {actual_type:?}");
    }

    ActorRef {
        actor_rc,
        _marker: PhantomData,
    }
}

/// Attempts to get a guard providing mutable access to the registered actor.
///
/// Returns `None` if the actor is not found or the type doesn't match.
///
/// # Safety
///
/// See [`get_actor_unchecked`] for safety requirements. The same aliasing
/// and thread-safety constraints apply.
#[must_use]
pub fn try_get_actor_unchecked<T: Actor>(id: &Ustr) -> Option<ActorRef<T>> {
    let registry = get_actor_registry();
    let actor_rc = registry.get(id)?;

    // SAFETY: Get a reference to check the type before casting
    let actor_ref = unsafe { &*actor_rc.get() };
    let actual_type = actor_ref.as_any().type_id();
    let expected_type = TypeId::of::<T>();

    if actual_type != expected_type {
        return None;
    }

    Some(ActorRef {
        actor_rc,
        _marker: PhantomData,
    })
}

/// Checks if an actor with the `id` exists in the registry.
pub fn actor_exists(id: &Ustr) -> bool {
    get_actor_registry().contains(id)
}

/// Returns the number of registered actors.
pub fn actor_count() -> usize {
    get_actor_registry().len()
}

#[cfg(test)]
/// Clears the actor registry (for test isolation).
pub fn clear_actor_registry() {
    let registry = get_actor_registry();
    registry.actors.borrow_mut().clear();
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::any::Any;

    use rstest::rstest;

    use super::*;

    #[derive(Debug)]
    struct TestActor {
        id: Ustr,
        value: i32,
    }

    impl Actor for TestActor {
        fn id(&self) -> Ustr {
            self.id
        }
        fn handle(&mut self, _msg: &dyn Any) {}
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[rstest]
    fn test_register_and_get_actor() {
        clear_actor_registry();

        let id = Ustr::from("test-actor");
        let actor = TestActor { id, value: 42 };
        register_actor(actor);

        let actor_ref = get_actor_unchecked::<TestActor>(&id);
        assert_eq!(actor_ref.value, 42);
    }

    #[rstest]
    fn test_mutation_through_reference() {
        clear_actor_registry();

        let id = Ustr::from("test-actor-mut");
        let actor = TestActor { id, value: 0 };
        register_actor(actor);

        let mut actor_ref = get_actor_unchecked::<TestActor>(&id);
        actor_ref.value = 999;

        let actor_ref2 = get_actor_unchecked::<TestActor>(&id);
        assert_eq!(actor_ref2.value, 999);
    }

    #[rstest]
    fn test_try_get_returns_none_for_missing() {
        clear_actor_registry();

        let id = Ustr::from("nonexistent");
        let result = try_get_actor_unchecked::<TestActor>(&id);
        assert!(result.is_none());
    }

    #[rstest]
    fn test_try_get_returns_none_for_wrong_type() {
        clear_actor_registry();

        #[derive(Debug)]
        struct OtherActor {
            id: Ustr,
        }

        impl Actor for OtherActor {
            fn id(&self) -> Ustr {
                self.id
            }
            fn handle(&mut self, _msg: &dyn Any) {}
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let id = Ustr::from("other-actor");
        let actor = OtherActor { id };
        register_actor(actor);

        let result = try_get_actor_unchecked::<TestActor>(&id);
        assert!(result.is_none());
    }
}
