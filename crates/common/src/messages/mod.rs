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

//! Message types for system communication.
//! 系统通信的消息类型。
//!
//! This module provides message types used for communication between different
//! parts of the NautilusTrader system, including data requests, execution commands,
//! and system control messages.
//! 此模块提供用于 NautilusTrader 系统不同部分之间通信的消息类型，
//! 包括数据请求、执行命令和系统控制消息。

use nautilus_model::{
    data::Data,
    events::{AccountState, OrderEventAny},
    instruments::InstrumentAny,
};
use strum::Display;

pub mod data;
pub mod execution;
pub mod system;

#[cfg(feature = "defi")]
pub mod defi;

// Re-exports
pub use data::{DataResponse, SubscribeCommand, UnsubscribeCommand};
pub use execution::ExecutionReport;

// TODO: Refine this to reduce disparity between enum sizes
// TODO: 优化此枚举以减少枚举大小之间的差异
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Display)]
pub enum DataEvent {
    Response(DataResponse),
    Data(Data),
    Instrument(InstrumentAny), // TODO: Eventually this can be `Data` once Cython is gone
                                // TODO: 一旦 Cython 被移除，这最终可以是 `Data`
    // nautilus-import-ok: conditional compilation import
    #[cfg(feature = "defi")]
    DeFi(nautilus_model::defi::data::DefiData),
}

/// Execution event variants for order events and reports.
/// 订单事件和报告的执行事件变体。
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Display)]
pub enum ExecutionEvent {
    Order(OrderEventAny),
    Report(ExecutionReport),
    Account(AccountState),
}
