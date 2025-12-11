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

//! Core constants.
//! 核心常量。

use std::env;

/// The NautilusTrader string constant.
/// NautilusTrader 字符串常量。
pub static NAUTILUS_TRADER: &str = "NautilusTrader";

/// The NautilusTrader version string read from the top-level `pyproject.toml` at compile time.
/// 在编译时从顶级 `pyproject.toml` 读取的 NautilusTrader 版本字符串。
pub static NAUTILUS_VERSION: &str = env!("NAUTILUS_VERSION");

/// The NautilusTrader common User-Agent string including the current version at compile time.
/// NautilusTrader 通用 User-Agent 字符串，包含编译时的当前版本。
pub static NAUTILUS_USER_AGENT: &str = env!("NAUTILUS_USER_AGENT");
