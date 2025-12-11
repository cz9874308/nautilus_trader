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

//! Technical analysis indicators for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的技术分析指标。
//!
//! The `nautilus-indicators` crate provides a comprehensive collection of technical analysis indicators
//! for quantitative trading and market research. This includes a wide variety of indicators
//! organized by category, with a unified trait-based architecture for consistent usage:
//! `nautilus-indicators` crate 提供全面的技术分析指标集合，用于量化交易和市场研究。
//! 这包括按类别组织的各种指标，具有统一的基于 trait 的架构以实现一致的使用：
//!
//! - **Moving averages**: SMA, EMA, DEMA, HMA, WMA, VWAP, adaptive averages, and linear regression.
//! - **移动平均线**：SMA、EMA、DEMA、HMA、WMA、VWAP、自适应平均和线性回归。
//! - **Momentum indicators**: RSI, MACD, Aroon, Bollinger Bands, CCI, Stochastics, and rate of change.
//! - **动量指标**：RSI、MACD、Aroon、布林带、CCI、随机指标和变化率。
//! - **Volatility indicators**: ATR, Donchian Channels, Keltner Channels, and volatility ratios.
//! - **波动率指标**：ATR、唐奇安通道、肯特纳通道和波动率比率。
//! - **Ratio analysis**: Efficiency ratios and spread analysis for relative performance.
//! - **比率分析**：用于相对性能的效率比率和价差分析。
//! - **Order book indicators**: Book imbalance ratio for analyzing market microstructure.
//! - **订单簿指标**：用于分析市场微观结构的订单簿不平衡比率。
//! - **Common indicator trait**: Unified interface supporting bars, quotes, trades, and order book data.
//! - **通用指标 trait**：支持 K 线、报价、交易和订单簿数据的统一接口。
//!
//! All indicators are designed for high-performance real-time processing with bounded memory
//! usage and efficient circular buffer implementations. The crate supports both Rust-native
//! usage and Python integration for strategy development and backtesting.
//! 所有指标都设计用于高性能实时处理，具有有界内存使用和高效的循环缓冲区实现。
//! 此 crate 支持 Rust 原生使用和 Python 集成，用于策略开发和回测。
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

pub mod average;
pub mod book;
pub mod indicator;
pub mod momentum;
pub mod ratio;
pub mod testing;
pub mod volatility;

#[cfg(test)]
mod stubs;

#[cfg(feature = "python")]
pub mod python;
