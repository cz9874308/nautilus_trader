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

//! Portfolio management and risk analysis for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的投资组合管理和风险分析。
//!
//! The `nautilus-portfolio` crate provides comprehensive portfolio management capabilities including
//! real-time position tracking, performance calculations, and risk management. This includes
//! sophisticated portfolio analytics and multi-currency support:
//! `nautilus-portfolio` crate 提供全面的投资组合管理功能，
//! 包括实时持仓跟踪、性能计算和风险管理。这包括复杂的投资组合分析和多货币支持：
//!
//! - **Portfolio tracking**: Real-time portfolio state management with position and balance monitoring.
//! - **投资组合跟踪**：具有持仓和余额监控的实时投资组合状态管理。
//! - **Account management**: Support for cash and margin accounts across multiple venues.
//! - **账户管理**：支持跨多个场所的现金和保证金账户。
//! - **Performance calculations**: Real-time unrealized PnL, realized PnL, and mark-to-market valuations.
//! - **性能计算**：实时未实现盈亏、已实现盈亏和按市值计价估值。
//! - **Risk management**: Initial margin calculations, maintenance margin tracking, and exposure monitoring.
//! - **风险管理**：初始保证金计算、维持保证金跟踪和风险敞口监控。
//! - **Multi-currency support**: Currency conversion and cross-currency risk exposure analysis.
//! - **多货币支持**：货币转换和跨货币风险敞口分析。
//! - **Configuration options**: Flexible settings for price types, currency conversion, and portfolio behavior.
//! - **配置选项**：价格类型、货币转换和投资组合行为的灵活设置。
//!
//! The crate handles complex portfolio scenarios including multi-venue trading, currency conversions,
//! and sophisticated margin calculations for both live trading and backtesting environments.
//! 此 crate 处理复杂的投资组合场景，包括多场所交易、货币转换，
//! 以及用于实时交易和回测环境的复杂保证金计算。
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

pub mod config;
pub mod manager;
pub mod portfolio;

#[cfg(test)]
mod tests;

// Re-exports
pub use portfolio::Portfolio;
