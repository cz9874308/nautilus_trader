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

//! Risk engine for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的风险引擎。
//!
//! The `nautilus-risk` crate provides comprehensive risk management capabilities including pre-trade
//! order validation, position sizing calculations, and trading controls. This system ensures
//! trading operations remain within defined risk parameters and regulatory constraints:
//! `nautilus-risk` crate 提供全面的风险管理功能，包括交易前订单验证、持仓规模计算和交易控制。
//! 此系统确保交易操作保持在定义的风险参数和监管约束内：
//!
//! - **Risk engine**: Central risk management orchestration with configurable trading states.
//! - **风险引擎**：具有可配置交易状态的中心风险管理编排。
//! - **Order validation**: Pre-trade checks for price, quantity, notional limits, and market conditions.
//! - **订单验证**：价格、数量、名义限额和市场条件的交易前检查。
//! - **Position sizing**: Fixed-risk position sizing calculations with commission and exchange rate support.
//! - **持仓规模**：具有佣金和汇率支持的固定风险持仓规模计算。
//! - **Trading controls**: Rate limiting, balance validation, and exposure management.
//! - **交易控制**：速率限制、余额验证和风险敞口管理。
//! - **Account protection**: Multi-currency balance checks and margin requirement validation.
//! - **账户保护**：多货币余额检查和保证金要求验证。
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

pub mod engine;
pub mod sizing;

// Re-exports
pub use engine::RiskEngine;
