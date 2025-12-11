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

//! System-level components and orchestration for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的系统级组件和编排。
//!
//! The `nautilus-system` crate provides the core system architecture for orchestrating trading systems,
//! including the kernel that manages all engines, configuration management,
//! and system-level factories for creating components:
//! `nautilus-system` crate 提供编排交易系统的核心系统架构，
//! 包括管理所有引擎的内核、配置管理和用于创建组件的系统级工厂：
//!
//! - `NautilusKernel` - Core system orchestrator managing engines and components.
//!   `NautilusKernel` - 管理引擎和组件的核心系统编排器。
//! - `NautilusKernelConfig` - Configuration for kernel initialization.
//!   `NautilusKernelConfig` - 内核初始化的配置。
//! - System builders and factories for component creation.
//!   用于创建组件的系统构建器和工厂。
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
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//!   `python`：启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
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

pub mod builder;
pub mod config;
pub mod factories;
pub mod kernel;
pub mod trader;

#[cfg(feature = "python")]
pub mod python;

// Re-exports
pub use builder::NautilusKernelBuilder;
pub use config::NautilusKernelConfig;
pub use factories::{ClientConfig, DataClientFactory, ExecutionClientFactory};
pub use kernel::NautilusKernel;
#[cfg(feature = "python")]
pub use python::{FactoryRegistry, get_global_pyo3_registry};
