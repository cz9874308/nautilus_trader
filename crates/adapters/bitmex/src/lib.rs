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

//! [NautilusTrader](http://nautilustrader.io) adapter for the [BitMEX](https://bitmex.com) cryptocurrency exchange.
//! [NautilusTrader](http://nautilustrader.io) 的 [BitMEX](https://bitmex.com) 加密货币交易所适配器。
//!
//! The `nautilus-bitmex` crate provides client bindings (HTTP & WebSocket), data
//! models and helper utilities that wrap the official **BitMEX API**.
//! `nautilus-bitmex` crate 提供客户端绑定（HTTP 和 WebSocket）、数据模型和辅助实用工具，
//! 这些工具封装了官方的 **BitMEX API**。
//!
//! The official BitMEX API reference can be found at <https://www.bitmex.com/app/apiOverview>.
//! All public links inside this crate reference the official documentation.
//! 官方 BitMEX API 参考可以在 <https://www.bitmex.com/app/apiOverview> 找到。
//! 此 crate 内的所有公共链接都引用官方文档。
//!
//! # Platform
//! # 平台
//!
//! [NautilusTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
//! algorithmic trading platform, providing quantitative traders with the ability to backtest
//! portfolios of automated trading strategies on historical data with an event-driven engine,
//! and also deploy those same strategies live, with no code changes.
//! [NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级的算法交易平台，
//! 为量化交易者提供了使用事件驱动引擎在历史数据上回测自动化交易策略组合的能力，
//! 并且无需代码更改即可实时部署相同的策略。
//!
//! NautilusTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
//! highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.
//! NautilusTrader 的设计、架构和实现理念将软件的正确性和安全性放在最高优先级，
//! 旨在支持任务关键型交易系统回测和实时部署工作负载。
//!
//! # Feature flags
//! # 特性标志
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case, i.e. whether to provide Python bindings
//! for the [nautilus_trader](https://pypi.org/project/nautilus_trader) Python package,
//! or as part of a Rust only build.
//! 此 crate 提供特性标志，用于在编译期间控制源代码的包含，
//! 具体取决于预期的用例，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，
//! 或者作为纯 Rust 构建的一部分。
//!
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `python`: 通过 [PyO3](https://pyo3.rs) 启用 Python 绑定。
//! - `extension-module`: Builds as a Python extension module (used with `python`).
//! - `extension-module`: 构建为 Python 扩展模块（与 `python` 一起使用）。

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod common;
pub mod config;
pub mod data;
pub mod error;
pub mod execution;
pub mod http;
pub mod websocket;

#[cfg(feature = "python")]
pub mod python;

// Re-exports
pub use crate::{
    common::{consts::BITMEX_VENUE, enums::BitmexSide},
    data::BitmexDataClient,
    execution::{
        BitmexExecutionClient,
        canceller::{BroadcasterMetrics, CancelBroadcaster, CancelBroadcasterConfig, ClientStats},
        submitter::{SubmitBroadcaster, SubmitBroadcasterConfig},
    },
    http::{client::BitmexHttpClient, error::BitmexHttpError},
    websocket::{client::BitmexWebSocketClient, error::BitmexWsError},
};
