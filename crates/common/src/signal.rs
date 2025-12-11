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

//! A user signal type.
//! 用户信号类型。

use nautilus_core::UnixNanos;
use serde::{Deserialize, Serialize};
use ustr::Ustr;

/// Represents a generic signal.
/// 表示通用信号。
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.common")
)]
pub struct Signal {
    /// The signal name.
    /// 信号名称。
    pub name: Ustr,
    /// The signal value.
    /// 信号值。
    pub value: String,
    /// UNIX timestamp (nanoseconds) when the signal event occurred.
    /// 信号事件发生时的 UNIX 时间戳（纳秒）。
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was created.
    /// 实例创建时的 UNIX 时间戳（纳秒）。
    pub ts_init: UnixNanos,
}

impl Signal {
    /// Creates a new [`Signal`] instance.
    /// 创建一个新的 [`Signal`] 实例。
    #[must_use]
    pub const fn new(name: Ustr, value: String, ts_event: UnixNanos, ts_init: UnixNanos) -> Self {
        Self {
            name,
            value,
            ts_event,
            ts_init,
        }
    }
}
