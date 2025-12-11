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

//! Component system for managing stateful system entities.
//! 用于管理有状态系统实体的组件系统。
//!
//! This module provides the component framework for managing the lifecycle and state
//! of system entities. Components have defined states (pre-initialized, ready, running,
//! stopped, etc.) and provide a consistent interface for state management and transitions.
//! 此模块提供用于管理系统实体的生命周期和状态的组件框架。
//! 组件具有定义的状态（预初始化、就绪、运行、停止等），并提供用于状态管理和转换的一致接口。

#![allow(unsafe_code)]

use std::{
    cell::{RefCell, UnsafeCell},
    fmt::Debug,
    rc::Rc,
};

use ahash::{AHashMap, AHashSet};
use nautilus_model::identifiers::{ComponentId, TraderId};
use ustr::Ustr;

use crate::{
    actor::{Actor, registry::get_actor_registry},
    cache::Cache,
    clock::Clock,
    enums::{ComponentState, ComponentTrigger},
};

/// Components have state and lifecycle management capabilities.
/// 组件具有状态和生命周期管理能力。
pub trait Component {
    /// Returns the unique identifier for this component.
    /// 返回此组件的唯一标识符。
    fn component_id(&self) -> ComponentId;

    /// Returns the current state of the component.
    /// 返回组件的当前状态。
    fn state(&self) -> ComponentState;

    /// Transition the component with the state trigger.
    /// 使用状态触发器转换组件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the `trigger` is an invalid transition from the current state.
    /// 如果 `trigger` 是从当前状态的无效转换，则返回错误。
    fn transition_state(&mut self, trigger: ComponentTrigger) -> anyhow::Result<()>;

    /// Returns whether the component is ready.
    /// 返回组件是否就绪。
    fn is_ready(&self) -> bool {
        self.state() == ComponentState::Ready
    }

    /// Returns whether the component is *not* running.
    /// 返回组件是否*未*运行。
    fn not_running(&self) -> bool {
        !self.is_running()
    }

    /// Returns whether the component is running.
    /// 返回组件是否正在运行。
    fn is_running(&self) -> bool {
        self.state() == ComponentState::Running
    }

    /// Returns whether the component is stopped.
    /// 返回组件是否已停止。
    fn is_stopped(&self) -> bool {
        self.state() == ComponentState::Stopped
    }

    /// Returns whether the component has been degraded.
    /// 返回组件是否已降级。
    fn is_degraded(&self) -> bool {
        self.state() == ComponentState::Degraded
    }

    /// Returns whether the component has been faulted.
    /// 返回组件是否已故障。
    fn is_faulted(&self) -> bool {
        self.state() == ComponentState::Faulted
    }

    /// Returns whether the component has been disposed.
    /// 返回组件是否已释放。
    fn is_disposed(&self) -> bool {
        self.state() == ComponentState::Disposed
    }

    /// Registers the component with a system.
    /// 将组件注册到系统。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the component fails to register.
    /// 如果组件注册失败，则返回错误。
    fn register(
        &mut self,
        trader_id: TraderId,
        clock: Rc<RefCell<dyn Clock>>,
        cache: Rc<RefCell<Cache>>,
    ) -> anyhow::Result<()>;

    /// Initializes the component.
    /// 初始化组件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the initialization state transition fails.
    /// 如果初始化状态转换失败，则返回错误。
    fn initialize(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Initialize)
    }

    /// Starts the component.
    /// 启动组件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the component fails to start.
    /// 如果组件启动失败，则返回错误。
    fn start(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Start)?; // -> Starting
                                                          // -> 正在启动

        if let Err(e) = self.on_start() {
            log_error(&e);
            return Err(e); // Halt state transition
                            // 停止状态转换
        }

        self.transition_state(ComponentTrigger::StartCompleted)?;

        Ok(())
    }

    /// Stops the component.
    /// 停止组件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the component fails to stop.
    /// 如果组件停止失败，则返回错误。
    fn stop(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Stop)?; // -> Stopping
                                                         // -> 正在停止

        if let Err(e) = self.on_stop() {
            log_error(&e);
            return Err(e); // Halt state transition
                            // 停止状态转换
        }

        self.transition_state(ComponentTrigger::StopCompleted)?;

        Ok(())
    }

    /// Resumes the component.
    /// 恢复组件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the component fails to resume.
    fn resume(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Resume)?; // -> Resuming

        if let Err(e) = self.on_resume() {
            log_error(&e);
            return Err(e); // Halt state transition
        }

        self.transition_state(ComponentTrigger::ResumeCompleted)?;

        Ok(())
    }

    /// Degrades the component.
    ///
    /// # Errors
    ///
    /// Returns an error if the component fails to degrade.
    fn degrade(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Degrade)?; // -> Degrading

        if let Err(e) = self.on_degrade() {
            log_error(&e);
            return Err(e); // Halt state transition
        }

        self.transition_state(ComponentTrigger::DegradeCompleted)?;

        Ok(())
    }

    /// Faults the component.
    ///
    /// # Errors
    ///
    /// Returns an error if the component fails to fault.
    fn fault(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Fault)?; // -> Faulting

        if let Err(e) = self.on_fault() {
            log_error(&e);
            return Err(e); // Halt state transition
        }

        self.transition_state(ComponentTrigger::FaultCompleted)?;

        Ok(())
    }

    /// Resets the component to its initial state.
    ///
    /// # Errors
    ///
    /// Returns an error if the component fails to reset.
    fn reset(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Reset)?; // -> Resetting

        if let Err(e) = self.on_reset() {
            log_error(&e);
            return Err(e); // Halt state transition
        }

        self.transition_state(ComponentTrigger::ResetCompleted)?;

        Ok(())
    }

    /// Disposes of the component, releasing any resources.
    ///
    /// # Errors
    ///
    /// Returns an error if the component fails to dispose.
    fn dispose(&mut self) -> anyhow::Result<()> {
        self.transition_state(ComponentTrigger::Dispose)?; // -> Disposing

        if let Err(e) = self.on_dispose() {
            log_error(&e);
            return Err(e); // Halt state transition
        }

        self.transition_state(ComponentTrigger::DisposeCompleted)?;

        Ok(())
    }

    /// Actions to be performed on start.
    ///
    /// # Errors
    ///
    /// Returns an error if starting the actor fails.
    fn on_start(&mut self) -> anyhow::Result<()> {
        log::warn!(
            "The `on_start` handler was called when not overridden, \
            it's expected that any actions required when stopping the component \
            occur here, such as unsubscribing from data",
        );
        Ok(())
    }

    /// Actions to be performed on stop.
    ///
    /// # Errors
    ///
    /// Returns an error if stopping the actor fails.
    fn on_stop(&mut self) -> anyhow::Result<()> {
        log::warn!(
            "The `on_stop` handler was called when not overridden, \
            it's expected that any actions required when stopping the component \
            occur here, such as unsubscribing from data",
        );
        Ok(())
    }

    /// Actions to be performed on resume.
    ///
    /// # Errors
    ///
    /// Returns an error if resuming the actor fails.
    fn on_resume(&mut self) -> anyhow::Result<()> {
        log::warn!(
            "The `on_resume` handler was called when not overridden, \
            it's expected that any actions required when resuming the component \
            following a stop occur here"
        );
        Ok(())
    }

    /// Actions to be performed on reset.
    ///
    /// # Errors
    ///
    /// Returns an error if resetting the actor fails.
    fn on_reset(&mut self) -> anyhow::Result<()> {
        log::warn!(
            "The `on_reset` handler was called when not overridden, \
            it's expected that any actions required when resetting the component \
            occur here, such as resetting indicators and other state"
        );
        Ok(())
    }

    /// Actions to be performed on dispose.
    ///
    /// # Errors
    ///
    /// Returns an error if disposing the actor fails.
    fn on_dispose(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Actions to be performed on degrade.
    ///
    /// # Errors
    ///
    /// Returns an error if degrading the actor fails.
    fn on_degrade(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Actions to be performed on fault.
    ///
    /// # Errors
    ///
    /// Returns an error if faulting the actor fails.
    fn on_fault(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

fn log_error(e: &anyhow::Error) {
    log::error!("{e}");
}

#[rustfmt::skip]
impl ComponentState {
    /// Transition the state machine with the component `trigger`.
    ///
    /// # Errors
    ///
    /// Returns an error if `trigger` is invalid for the current state.
    pub fn transition(&mut self, trigger: &ComponentTrigger) -> anyhow::Result<Self> {
        let new_state = match (&self, trigger) {
            (Self::PreInitialized, ComponentTrigger::Initialize) => Self::Ready,
            (Self::Ready, ComponentTrigger::Reset) => Self::Resetting,
            (Self::Ready, ComponentTrigger::Start) => Self::Starting,
            (Self::Ready, ComponentTrigger::Dispose) => Self::Disposing,
            (Self::Resetting, ComponentTrigger::ResetCompleted) => Self::Ready,
            (Self::Starting, ComponentTrigger::StartCompleted) => Self::Running,
            (Self::Starting, ComponentTrigger::Stop) => Self::Stopping,
            (Self::Starting, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Running, ComponentTrigger::Stop) => Self::Stopping,
            (Self::Running, ComponentTrigger::Degrade) => Self::Degrading,
            (Self::Running, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Resuming, ComponentTrigger::Stop) => Self::Stopping,
            (Self::Resuming, ComponentTrigger::ResumeCompleted) => Self::Running,
            (Self::Resuming, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Stopping, ComponentTrigger::StopCompleted) => Self::Stopped,
            (Self::Stopping, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Stopped, ComponentTrigger::Reset) => Self::Resetting,
            (Self::Stopped, ComponentTrigger::Resume) => Self::Resuming,
            (Self::Stopped, ComponentTrigger::Dispose) => Self::Disposing,
            (Self::Stopped, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Degrading, ComponentTrigger::DegradeCompleted) => Self::Degraded,
            (Self::Degraded, ComponentTrigger::Resume) => Self::Resuming,
            (Self::Degraded, ComponentTrigger::Stop) => Self::Stopping,
            (Self::Degraded, ComponentTrigger::Fault) => Self::Faulting,
            (Self::Disposing, ComponentTrigger::DisposeCompleted) => Self::Disposed,
            (Self::Faulting, ComponentTrigger::FaultCompleted) => Self::Faulted,
            _ => anyhow::bail!("Invalid state trigger {self} -> {trigger}"),
        };
        Ok(new_state)
    }
}

thread_local! {
    static COMPONENT_REGISTRY: ComponentRegistry = ComponentRegistry::new();
}

/// Registry for storing components with runtime borrow tracking.
///
/// The registry tracks which components are currently mutably borrowed to prevent
/// multiple simultaneous mutable borrows (which would be undefined behavior).
pub struct ComponentRegistry {
    components: RefCell<AHashMap<Ustr, Rc<UnsafeCell<dyn Component>>>>,
    borrows: RefCell<AHashSet<Ustr>>,
}

impl Debug for ComponentRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let components_ref = self.components.borrow();
        let keys: Vec<&Ustr> = components_ref.keys().collect();
        f.debug_struct(stringify!(ComponentRegistry))
            .field("components", &keys)
            .field("active_borrows", &self.borrows.borrow().len())
            .finish()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: RefCell::new(AHashMap::new()),
            borrows: RefCell::new(AHashSet::new()),
        }
    }

    pub fn insert(&self, id: Ustr, component: Rc<UnsafeCell<dyn Component>>) {
        self.components.borrow_mut().insert(id, component);
    }

    pub fn get(&self, id: &Ustr) -> Option<Rc<UnsafeCell<dyn Component>>> {
        self.components.borrow().get(id).cloned()
    }

    /// Checks if a component is currently borrowed.
    pub fn is_borrowed(&self, id: &Ustr) -> bool {
        self.borrows.borrow().contains(id)
    }

    /// Marks a component as borrowed. Returns false if already borrowed.
    fn try_borrow(&self, id: Ustr) -> bool {
        let mut borrows = self.borrows.borrow_mut();
        if borrows.contains(&id) {
            false
        } else {
            borrows.insert(id);
            true
        }
    }

    /// Releases a borrow on a component.
    fn release_borrow(&self, id: &Ustr) {
        self.borrows.borrow_mut().remove(id);
    }
}

/// Guard that releases a component borrow when dropped.
///
/// This ensures borrows are released even if the code panics during
/// a lifecycle method call.
struct BorrowGuard {
    id: Ustr,
}

impl BorrowGuard {
    fn new(id: Ustr) -> Self {
        Self { id }
    }
}

impl Drop for BorrowGuard {
    fn drop(&mut self) {
        get_component_registry().release_borrow(&self.id);
    }
}

/// Returns a reference to the global component registry.
pub fn get_component_registry() -> &'static ComponentRegistry {
    COMPONENT_REGISTRY.with(|registry| unsafe {
        // SAFETY: We return a static reference that lives for the lifetime of the thread.
        // Since this is thread_local storage, each thread has its own instance.
        std::mem::transmute::<&ComponentRegistry, &'static ComponentRegistry>(registry)
    })
}

/// Registers a component.
pub fn register_component<T>(component: T) -> Rc<UnsafeCell<T>>
where
    T: Component + 'static,
{
    let component_id = component.component_id().inner();
    let component_ref = Rc::new(UnsafeCell::new(component));

    // Register in component registry
    let component_trait_ref: Rc<UnsafeCell<dyn Component>> = component_ref.clone();
    get_component_registry().insert(component_id, component_trait_ref);

    component_ref
}

/// Registers a component that also implements Actor.
pub fn register_component_actor<T>(component: T) -> Rc<UnsafeCell<T>>
where
    T: Component + Actor + 'static,
{
    let component_id = component.component_id().inner();
    let actor_id = component.id();
    let component_ref = Rc::new(UnsafeCell::new(component));

    // Register in component registry
    let component_trait_ref: Rc<UnsafeCell<dyn Component>> = component_ref.clone();
    get_component_registry().insert(component_id, component_trait_ref);

    // Register in actor registry
    let actor_trait_ref: Rc<UnsafeCell<dyn Actor>> = component_ref.clone();
    get_actor_registry().insert(actor_id, actor_trait_ref);

    component_ref
}

/// Safely calls start() on a component in the global registry.
///
/// # Errors
///
/// - Returns an error if the component is not found.
/// - Returns an error if the component is already borrowed.
/// - Returns an error if start() fails.
pub fn start_component(id: &Ustr) -> anyhow::Result<()> {
    let registry = get_component_registry();
    let component_ref = registry
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("Component '{id}' not found in global registry"))?;

    if !registry.try_borrow(*id) {
        anyhow::bail!(
            "Component '{id}' is already mutably borrowed. \
             This would create aliasing mutable references (undefined behavior)."
        );
    }

    let _guard = BorrowGuard::new(*id);

    // SAFETY: Borrow tracking ensures exclusive access
    unsafe {
        let component = &mut *component_ref.get();
        component.start()
    }
}

/// Safely calls stop() on a component in the global registry.
///
/// # Errors
///
/// - Returns an error if the component is not found.
/// - Returns an error if the component is already borrowed.
/// - Returns an error if stop() fails.
pub fn stop_component(id: &Ustr) -> anyhow::Result<()> {
    let registry = get_component_registry();
    let component_ref = registry
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("Component '{id}' not found in global registry"))?;

    if !registry.try_borrow(*id) {
        anyhow::bail!(
            "Component '{id}' is already mutably borrowed. \
             This would create aliasing mutable references (undefined behavior)."
        );
    }

    let _guard = BorrowGuard::new(*id);

    // SAFETY: Borrow tracking ensures exclusive access
    unsafe {
        let component = &mut *component_ref.get();
        component.stop()
    }
}

/// Safely calls reset() on a component in the global registry.
///
/// # Errors
///
/// - Returns an error if the component is not found.
/// - Returns an error if the component is already borrowed.
/// - Returns an error if reset() fails.
pub fn reset_component(id: &Ustr) -> anyhow::Result<()> {
    let registry = get_component_registry();
    let component_ref = registry
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("Component '{id}' not found in global registry"))?;

    if !registry.try_borrow(*id) {
        anyhow::bail!(
            "Component '{id}' is already mutably borrowed. \
             This would create aliasing mutable references (undefined behavior)."
        );
    }

    let _guard = BorrowGuard::new(*id);

    // SAFETY: Borrow tracking ensures exclusive access
    unsafe {
        let component = &mut *component_ref.get();
        component.reset()
    }
}

/// Safely calls dispose() on a component in the global registry.
///
/// # Errors
///
/// - Returns an error if the component is not found.
/// - Returns an error if the component is already borrowed.
/// - Returns an error if dispose() fails.
pub fn dispose_component(id: &Ustr) -> anyhow::Result<()> {
    let registry = get_component_registry();
    let component_ref = registry
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("Component '{id}' not found in global registry"))?;

    if !registry.try_borrow(*id) {
        anyhow::bail!(
            "Component '{id}' is already mutably borrowed. \
             This would create aliasing mutable references (undefined behavior)."
        );
    }

    let _guard = BorrowGuard::new(*id);

    // SAFETY: Borrow tracking ensures exclusive access
    unsafe {
        let component = &mut *component_ref.get();
        component.dispose()
    }
}

/// Returns a component from the global registry by ID.
pub fn get_component(id: &Ustr) -> Option<Rc<UnsafeCell<dyn Component>>> {
    get_component_registry().get(id)
}

#[cfg(test)]
/// Clears the component registry (for test isolation).
pub fn clear_component_registry() {
    let registry = get_component_registry();
    registry.components.borrow_mut().clear();
    registry.borrows.borrow_mut().clear();
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use rstest::rstest;

    use super::*;

    struct TestComponent {
        id: ComponentId,
        state: ComponentState,
        should_panic: &'static AtomicBool,
    }

    impl TestComponent {
        fn new(name: &str, should_panic: &'static AtomicBool) -> Self {
            Self {
                id: ComponentId::new(name),
                state: ComponentState::Ready,
                should_panic,
            }
        }
    }

    impl Component for TestComponent {
        fn component_id(&self) -> ComponentId {
            self.id
        }

        fn state(&self) -> ComponentState {
            self.state
        }

        fn transition_state(&mut self, trigger: ComponentTrigger) -> anyhow::Result<()> {
            self.state = self.state.transition(&trigger)?;
            Ok(())
        }

        fn register(
            &mut self,
            _trader_id: TraderId,
            _clock: Rc<RefCell<dyn Clock>>,
            _cache: Rc<RefCell<Cache>>,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        #[allow(clippy::panic_in_result_fn)] // Intentional panic for testing
        fn on_start(&mut self) -> anyhow::Result<()> {
            if self.should_panic.load(Ordering::SeqCst) {
                panic!("Intentional panic for testing");
            }
            Ok(())
        }
    }

    static NO_PANIC: AtomicBool = AtomicBool::new(false);
    static DO_PANIC: AtomicBool = AtomicBool::new(true);

    #[rstest]
    fn test_component_borrow_tracking_prevents_double_borrow() {
        clear_component_registry();

        let id = Ustr::from("test-component-1");
        let component = TestComponent::new("test-component-1", &NO_PANIC);
        let component_id = component.id.inner();

        let component_ref = Rc::new(UnsafeCell::new(component));
        get_component_registry().insert(component_id, component_ref);

        // First borrow via start_component should succeed
        let result1 = start_component(&id);
        assert!(result1.is_ok());

        // Component should now be borrowable again (guard released)
        let result2 = stop_component(&id);
        assert!(result2.is_ok());
    }

    #[rstest]
    fn test_component_borrow_released_after_lifecycle_call() {
        clear_component_registry();

        let id = Ustr::from("test-component-2");
        let component = TestComponent::new("test-component-2", &NO_PANIC);
        let component_id = component.id.inner();

        let component_ref = Rc::new(UnsafeCell::new(component));
        get_component_registry().insert(component_id, component_ref);

        // Call start - borrow should be released after
        let _ = start_component(&id);

        // Verify not marked as borrowed
        assert!(!get_component_registry().is_borrowed(&id));
    }

    #[rstest]
    fn test_component_borrow_released_on_panic() {
        clear_component_registry();

        let id = Ustr::from("test-component-panic");
        let component = TestComponent::new("test-component-panic", &DO_PANIC);
        let component_id = component.id.inner();

        let component_ref = Rc::new(UnsafeCell::new(component));
        get_component_registry().insert(component_id, component_ref);

        // Call start which will panic - catch the panic
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = start_component(&id);
        }));
        assert!(result.is_err(), "Expected panic from on_start");

        // Borrow should still be released due to BorrowGuard drop
        assert!(
            !get_component_registry().is_borrowed(&id),
            "Borrow was not released after panic"
        );
    }
}
