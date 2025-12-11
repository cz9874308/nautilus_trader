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

//! Actor system for event-driven message processing.
//! 用于事件驱动消息处理的参与者系统。
//!
//! This module provides the actor framework used throughout NautilusTrader for handling
//! data processing, event management, and asynchronous message handling. Actors are
//! lightweight components that process messages in isolation.
//! 此模块提供在整个 NautilusTrader 中用于处理数据处理、事件管理和异步消息处理的参与者框架。
//! 参与者是隔离处理消息的轻量级组件。

#![allow(unsafe_code)]

use std::{any::Any, fmt::Debug};

use ustr::Ustr;

pub mod data_actor;
#[cfg(feature = "indicators")]
pub(crate) mod indicators;
pub mod registry;

#[cfg(test)]
mod tests;

// Re-exports
pub use data_actor::{DataActor, DataActorConfig, DataActorCore};

pub use crate::component::Component;

pub trait Actor: Any + Debug {
    /// The unique identifier for the actor.
    /// 参与者的唯一标识符。
    fn id(&self) -> Ustr;
    /// Handles the `msg`.
    /// 处理 `msg`。
    fn handle(&mut self, msg: &dyn Any);
    /// Returns a reference to `self` as `Any`, for downcasting support.
    /// 返回对 `self` 的引用作为 `Any`，用于向下转换支持。
    fn as_any(&self) -> &dyn Any;
    /// Returns a mutable reference to `self` as `Any`, for downcasting support.
    /// 返回对 `self` 的可变引用作为 `Any`，用于向下转换支持。
    ///
    /// Default implementation simply coerces `&mut Self` to `&mut dyn Any`.
    /// 默认实现简单地将 `&mut Self` 强制转换为 `&mut dyn Any`。
    ///
    /// # Note
    /// # 备注
    ///
    /// This method is not object-safe and thus only available on sized `Self`.
    /// 此方法不是对象安全的，因此仅在大小已知的 `Self` 上可用。
    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: Sized,
    {
        self
    }
}
