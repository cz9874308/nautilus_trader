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

//! Global runtime machinery and thread-local storage.
//! 全局运行时机制和线程本地存储。
//!
//! This module provides global access to shared runtime resources including clocks,
//! message queues, and time event channels. It manages thread-local storage for
//! system-wide components that need to be accessible across threads.
//! 此模块提供对共享运行时资源（包括时钟、消息队列和时间事件通道）的全局访问。
//! 它管理系统范围组件的线程本地存储，这些组件需要跨线程访问。

use std::{cell::OnceCell, fmt::Debug, sync::Arc};

use crate::{
    messages::{data::DataCommand, execution::TradingCommand},
    msgbus::{self, switchboard::MessagingSwitchboard},
    timer::TimeEventHandlerV2,
};

/// Trait for data command sending that can be implemented for both sync and async runners.
/// 用于数据命令发送的 trait，可以为同步和异步运行器实现。
pub trait DataCommandSender {
    /// Executes a data command.
    /// 执行数据命令。
    ///
    /// - **Sync runners** send the command to a queue for synchronous execution.
    ///   **同步运行器**将命令发送到队列以进行同步执行。
    /// - **Async runners** send the command to a channel for asynchronous execution.
    ///   **异步运行器**将命令发送到通道以进行异步执行。
    fn execute(&self, command: DataCommand);
}

/// Synchronous implementation of DataCommandSender for backtest environments.
/// 用于回测环境的 DataCommandSender 的同步实现。
#[derive(Debug)]
pub struct SyncDataCommandSender;

impl DataCommandSender for SyncDataCommandSender {
    fn execute(&self, command: DataCommand) {
        // TODO: Placeholder, we still need to queue and drain even for sync
        // TODO: 占位符，即使对于同步，我们仍然需要排队和排空
        let endpoint = MessagingSwitchboard::data_engine_execute();
        msgbus::send_any(endpoint, &command);
    }
}

/// Gets the global data command sender.
/// 获取全局数据命令发送器。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if the sender is uninitialized.
/// 如果发送器未初始化，则会 panic。
#[must_use]
pub fn get_data_cmd_sender() -> Arc<dyn DataCommandSender> {
    DATA_CMD_SENDER.with(|sender| {
        sender
            .get()
            .expect("Data command sender should be initialized by runner")
            .clone()
    })
}

/// Sets the global data command sender.
/// 设置全局数据命令发送器。
///
/// This should be called by the runner when it initializes.
/// Can only be called once per thread.
/// 这应该由运行器在初始化时调用。每个线程只能调用一次。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if a sender has already been set.
/// 如果已设置发送器，则会 panic。
pub fn set_data_cmd_sender(sender: Arc<dyn DataCommandSender>) {
    DATA_CMD_SENDER.with(|s| {
        if s.set(sender).is_err() {
            panic!("Data command sender can only be set once");
        }
    });
}

/// Trait for time event sending that can be implemented for both sync and async runners.
/// 用于时间事件发送的 trait，可以为同步和异步运行器实现。
pub trait TimeEventSender: Debug + Send + Sync {
    /// Sends a time event handler.
    /// 发送时间事件处理器。
    fn send(&self, handler: TimeEventHandlerV2);
}

/// Gets the global time event sender.
/// 获取全局时间事件发送器。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if the sender is uninitialized.
/// 如果发送器未初始化，则会 panic。
#[must_use]
pub fn get_time_event_sender() -> Arc<dyn TimeEventSender> {
    TIME_EVENT_SENDER.with(|sender| {
        sender
            .get()
            .expect("Time event sender should be initialized by runner")
            .clone()
    })
}

/// Attempts to get the global time event sender without panicking.
/// 尝试获取全局时间事件发送器而不 panic。
///
/// Returns `None` if the sender is not initialized (e.g., in test environments).
/// 如果发送器未初始化（例如，在测试环境中），则返回 `None`。
#[must_use]
pub fn try_get_time_event_sender() -> Option<Arc<dyn TimeEventSender>> {
    TIME_EVENT_SENDER.with(|sender| sender.get().cloned())
}

/// Sets the global time event sender.
/// 设置全局时间事件发送器。
///
/// Can only be called once per thread.
/// 每个线程只能调用一次。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if a sender has already been set.
/// 如果已设置发送器，则会 panic。
pub fn set_time_event_sender(sender: Arc<dyn TimeEventSender>) {
    TIME_EVENT_SENDER.with(|s| {
        if s.set(sender).is_err() {
            panic!("Time event sender can only be set once");
        }
    });
}

/// Trait for trading command sending that can be implemented for both sync and async runners.
/// 用于交易命令发送的 trait，可以为同步和异步运行器实现。
pub trait TradingCommandSender {
    /// Executes a trading command.
    /// 执行交易命令。
    ///
    /// - **Sync runners** send the command to a queue for synchronous execution.
    /// - **Async runners** send the command to a channel for asynchronous execution.
    fn execute(&self, command: TradingCommand);
}

/// Gets the global trading command sender.
///
/// # Panics
///
/// Panics if the sender is uninitialized.
#[must_use]
pub fn get_trading_cmd_sender() -> Arc<dyn TradingCommandSender> {
    EXEC_CMD_SENDER.with(|sender| {
        sender
            .get()
            .expect("Trading command sender should be initialized by runner")
            .clone()
    })
}

/// Sets the global trading command sender.
///
/// This should be called by the runner when it initializes.
/// Can only be called once per thread.
///
/// # Panics
///
/// Panics if a sender has already been set.
pub fn set_exec_cmd_sender(sender: Arc<dyn TradingCommandSender>) {
    EXEC_CMD_SENDER.with(|s| {
        if s.set(sender).is_err() {
            panic!("Trading command sender can only be set once");
        }
    });
}

thread_local! {
    static TIME_EVENT_SENDER: OnceCell<Arc<dyn TimeEventSender>> = const { OnceCell::new() };
    static DATA_CMD_SENDER: OnceCell<Arc<dyn DataCommandSender>> = const { OnceCell::new() };
    static EXEC_CMD_SENDER: OnceCell<Arc<dyn TradingCommandSender>> = const { OnceCell::new() };
}
