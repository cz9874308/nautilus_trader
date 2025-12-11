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

//! Blockchain data adapter for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的区块链数据适配器。
//!
//! The `nautilus-blockchain` crate provides a high-performance, universal, extensible adapter for ingesting
//! DeFi data from decentralized exchanges (DEXs), liquidity pools, and on-chain events.
//! It enables you to power analytics pipelines and trading strategies with real-time and historical
//! on-chain data.
//! `nautilus-blockchain` crate 提供了一个高性能、通用、可扩展的适配器，用于从去中心化交易所（DEX）、
//! 流动性池和链上事件中摄取 DeFi 数据。它使您能够使用实时和历史链上数据为分析管道和交易策略提供支持。
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
//! - `hypersync`: Enables the [HyperSync](https://envio.dev/#hypersync) client integration.
//! - `hypersync`: 启用 [HyperSync](https://envio.dev/#hypersync) 客户端集成。
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `python`: 启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
//! - `extension-module`: Builds as a Python extension module (used with `python`).
//! - `extension-module`: 构建为 Python 扩展模块（与 `python` 一起使用）。
//! - `turmoil`: Enables deterministic network simulation testing with [turmoil](https://github.com/tokio-rs/turmoil).
//! - `turmoil`: 启用使用 [turmoil](https://github.com/tokio-rs/turmoil) 的确定性网络模拟测试。

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod config;
pub mod constants;
pub mod contracts;
pub mod decode;
pub mod events;
pub mod math;
pub mod rpc;

#[cfg(feature = "hypersync")]
pub mod cache;

#[cfg(feature = "hypersync")]
pub mod execution;

#[cfg(feature = "hypersync")]
pub mod data;

#[cfg(feature = "hypersync")]
pub mod exchanges;

#[cfg(feature = "hypersync")]
pub mod factories;

#[cfg(feature = "hypersync")]
pub mod hypersync;

#[cfg(feature = "hypersync")]
pub mod services;

#[cfg(feature = "python")]
pub mod python;
