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

//! Order execution engine for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的订单执行引擎。
//!
//! The `nautilus-execution` crate provides a comprehensive order execution system that handles the complete
//! order lifecycle from submission to fill processing. This includes sophisticated order matching,
//! execution venue integration, and advanced order type emulation:
//! `nautilus-execution` crate 提供了一个全面的订单执行系统，处理从提交到成交处理的完整订单生命周期。
//! 这包括复杂的订单匹配、执行场所集成和高级订单类型模拟：
//!
//! - **Execution engine**: Central orchestration of order routing and position management.
//! - **执行引擎**：订单路由和持仓管理的中央编排。
//! - **Order matching engine**: High-fidelity market simulation for backtesting and paper trading.
//! - **订单匹配引擎**：用于回测和模拟交易的高保真度市场模拟。
//! - **Order emulator**: Advanced order types not natively supported by venues (trailing stops, contingent orders).
//! - **订单模拟器**：场所不原生支持的高级订单类型（跟踪止损、条件订单）。
//! - **Execution clients**: Abstract interfaces for connecting to trading venues and brokers.
//! - **执行客户端**：用于连接到交易场所和经纪商的抽象接口。
//! - **Order manager**: Local order lifecycle management and state tracking.
//! - **订单管理器**：本地订单生命周期管理和状态跟踪。
//! - **Matching core**: Low-level order book and price-time priority matching algorithms.
//! - **匹配核心**：低级订单簿和价格-时间优先级匹配算法。
//! - **Fee and fill models**: Configurable execution cost simulation and realistic fill behavior.
//! - **费用和成交模型**：可配置的执行成本模拟和真实的成交行为。
//!
//! The crate supports both live trading environments (with real execution clients) and simulated
//! environments (with matching engines), making it suitable for production trading, strategy
//! development, and comprehensive backtesting.
//! 此 crate 支持实时交易环境（使用真实执行客户端）和模拟环境（使用匹配引擎），
//! 使其适用于生产交易、策略开发和全面回测。
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

pub mod client;
pub mod engine;
pub mod matching_core;
pub mod matching_engine;
pub mod models;
pub mod order_emulator;
pub mod order_manager;
pub mod protection;
pub mod trailing;
