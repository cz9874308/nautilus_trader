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

//! Trading domain model for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的交易领域模型。
//!
//! The `nautilus-model` crate provides a type-safe domain model that forms the backbone of the
//! framework and can serve as the foundation for building algorithmic trading systems.
//! `nautilus-model` crate 提供类型安全的领域模型，构成框架的骨干，可以作为构建算法交易系统的基础。
//!
//! # Platform
//! # 平台
//!
//! [NautilusTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
//! algorithmic trading platform, providing quantitative traders with the ability to backtest
//! portfolios of automated trading strategies on historical data with an event-driven engine,
//! and also deploy those same strategies live, with no code changes.
//! [NautilusTrader](http://nautilustrader.io) 是一个开源、高性能、生产级的算法交易平台，
//! 为量化交易者提供使用事件驱动引擎在历史数据上回测自动化交易策略投资组合的能力，
//! 并且可以在不更改代码的情况下实时部署这些相同的策略。
//!
//! NautilusTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
//! highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.
//! NautilusTrader 的设计、架构和实施理念将软件正确性和安全性放在最高优先级，
//! 旨在支持关键任务的交易系统回测和实时部署工作负载。
//!
//! # Feature flags
//! # 特性标志
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case, i.e. whether to provide Python bindings
//! for the [nautilus_trader](https://pypi.org/project/nautilus_trader) Python package,
//! or as part of a Rust only build.
//! 此 crate 提供特性标志以在编译期间控制源代码的包含，取决于预期的用例，
//! 即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，
//! 或作为纯 Rust 构建的一部分。
//!
//! - `ffi`: Enables the C foreign function interface (FFI) from [cbindgen](https://github.com/mozilla/cbindgen).
//!   `ffi`：启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口（FFI）。
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//!   `python`：启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
//! - `stubs`: Enables type stubs for use in testing scenarios.
//!   `stubs`：启用用于测试场景的类型存根。
//! - `high-precision`: Enables [high-precision mode](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode) to use 128-bit value types.
//!   `high-precision`：启用[高精度模式](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode)以使用 128 位值类型。
//! - `defi`: Enables the DeFi (Decentralized Finance) domain model.
//!   `defi`：启用 DeFi（去中心化金融）领域模型。
//! - `extension-module`: Builds the crate as a Python extension module.
//!   `extension-module`：将 crate 构建为 Python 扩展模块。

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod accounts;
pub mod currencies;
pub mod data;
pub mod enums;
pub mod events;
pub mod identifiers;
pub mod instruments;
pub mod macros;
pub mod orderbook;
pub mod orders;
pub mod position;
pub mod reports;
pub mod types;
pub mod venues;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "python")]
pub mod python;

#[cfg(any(test, feature = "stubs"))]
pub mod stubs;

#[cfg(feature = "defi")]
pub mod defi;
