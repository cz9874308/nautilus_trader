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

//! Core foundational types and utilities for [NautilusTrader](http://nautilustrader.io).
//! [NautilusTrader](http://nautilustrader.io) 的核心基础类型和工具。
//!
//! The `nautilus-core` crate is designed to be lightweight, efficient, and to provide zero-cost abstractions
//! wherever possible. It supplies the essential building blocks used across the NautilusTrader
//! ecosystem, including:
//! `nautilus-core` crate 设计为轻量级、高效，并在可能的地方提供零成本抽象。
//! 它为整个 NautilusTrader 生态系统提供基础构建块，包括：
//!
//! - Time handling and atomic clock functionality.
//!   时间处理和原子时钟功能。
//! - UUID generation and management.
//!   UUID 生成和管理。
//! - Mathematical functions and interpolation utilities.
//!   数学函数和插值工具。
//! - Correctness validation functions.
//!   正确性验证函数。
//! - Serialization traits and helpers.
//!   序列化 trait 和辅助工具。
//! - Cross-platform environment utilities.
//!   跨平台环境工具。
//! - Abstractions over common collections.
//!   通用集合的抽象。
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
//! 此 crate 提供特性标志以在编译期间控制源代码的包含，
//! 取决于预期的用例，即是否为 [nautilus_trader](https://pypi.org/project/nautilus_trader) Python 包提供 Python 绑定，
//! 或作为纯 Rust 构建的一部分。
//!
//! - `ffi`: Enables the C foreign function interface (FFI) from [cbindgen](https://github.com/mozilla/cbindgen).
//!   `ffi`：启用来自 [cbindgen](https://github.com/mozilla/cbindgen) 的 C 外部函数接口（FFI）。
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//!   `python`：启用来自 [PyO3](https://pyo3.rs) 的 Python 绑定。
//! - `extension-module`: Builds the crate as a Python extension module.
//!   `extension-module`：将 crate 构建为 Python 扩展模块。

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod collections;
pub mod consts;
pub mod correctness;
pub mod datetime;
pub mod drop;
pub mod env;
pub mod math;
pub mod message;
pub mod nanos;

pub mod parsing;
pub mod paths;
pub mod serialization;
pub mod shared;
pub mod string;
pub mod time;
pub mod uuid;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "python")]
pub mod python;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("Unsupported platform: Nautilus supports only Linux, macOS, and Windows");

// Re-exports
// 重新导出
pub use crate::{
    drop::CleanDrop,
    nanos::UnixNanos,
    shared::{SharedCell, WeakCell},
    time::AtomicTime,
    uuid::UUID4,
};

/// Message for when a mutex guard cannot be acquired due to poisoning.
/// 当由于中毒（poisoning）而无法获取 mutex guard 时的消息。
///
/// Mutex guards should use `expect` rather than handle poison errors.
/// Mutex guard 应该使用 `expect` 而不是处理 poison 错误。
/// A poisoned mutex indicates a thread panicked while holding the lock,
/// meaning protected data may be in an inconsistent state. Propagating
/// the panic is the idiomatic and safe approach, as continuing with
/// potentially corrupted data would violate safety invariants.
/// 中毒的 mutex 表示持有锁的线程发生了 panic，
/// 意味着受保护的数据可能处于不一致的状态。传播 panic 是惯用且安全的方法，
/// 因为继续使用可能已损坏的数据会违反安全不变量。
pub const MUTEX_POISONED: &str = "Mutex poisoned";
