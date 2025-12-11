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

//! DeFi (Decentralized Finance) integration for the data crate.
//! 数据 crate 的 DeFi（去中心化金融）集成。
//!
//! This module provides centralized access to DeFi functionality throughout the data crate.
//! DeFi support includes client subscriptions and engine processing.
//! 此模块在整个数据 crate 中提供对 DeFi 功能的集中访问。
//! DeFi 支持包括客户端订阅和引擎处理。
//!
//! # Feature Flag
//! # 特性标志
//!
//! All DeFi functionality requires the `defi` feature flag to be enabled:
//! 所有 DeFi 功能都需要启用 `defi` 特性标志：
//! ```toml
//! nautilus-data = { version = "0.x", features = ["defi"] }
//! ```

pub mod client;
pub mod engine;
