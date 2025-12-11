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

//! Data engine and market data processing for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的数据引擎和市场数据处理。
//!
//! The `nautilus-data` crate provides a comprehensive framework for handling market data ingestion,
//! processing, and aggregation within the NautilusTrader ecosystem. This includes real-time
//! data streaming, historical data management, and various aggregation methodologies:
//! `nautilus-data` crate 提供了一个全面的框架，用于处理 NautilusTrader 生态系统内的
//! 市场数据摄取、处理和聚合。这包括实时数据流、历史数据管理和各种聚合方法：
//!
//! - High-performance data engine for orchestrating data operations.
//! - 用于编排数据操作的高性能数据引擎。
//! - Data client infrastructure for connecting to market data providers.
//! - 用于连接到市场数据提供商的数据客户端基础设施。
//! - Bar aggregation machinery supporting tick, volume, value, and time-based aggregation.
//! - 支持基于 tick、成交量、价值和时间的聚合的 K 线聚合机制。
//! - Order book management and delta processing capabilities.
//! - 订单簿管理和增量处理能力。
//! - Subscription management and data request handling.
//! - 订阅管理和数据请求处理。
//! - Configurable data routing and processing pipelines.
//! - 可配置的数据路由和处理管道。
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
//! - `ffi`: Enables the C foreign function interface (FFI) from [cbindgen](https://github.com/mozilla/cbindgen).
//! - `ffi`: 启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口 (FFI)。
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
//! - `high-precision`: Enables [high-precision mode](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode) to use 128-bit value types.
//! - `high-precision`: 启用 [高精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode) 以使用 128 位值类型。
//! - `defi`: Enables DeFi (Decentralized Finance) support.
//! - `defi`: 启用 DeFi（去中心化金融）支持。
//! - `extension-module`: Builds the crate as a Python extension module.
//! - `extension-module`: 将 crate 构建为 Python 扩展模块。

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod aggregation;
pub mod client;
pub mod engine;

#[cfg(feature = "defi")]
pub mod defi;

// Re-exports
pub use client::{DataClient, DataClientAdapter};
