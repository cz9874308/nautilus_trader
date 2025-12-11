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

//! Represents a valid trader ID.
//! 表示有效的交易者 ID。

use std::fmt::{Debug, Display, Formatter};

use nautilus_core::correctness::{FAILED, check_string_contains, check_valid_string_ascii};
use ustr::Ustr;

/// Represents a valid trader ID.
/// 表示有效的交易者 ID。
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.model")
)]
pub struct TraderId(Ustr);

impl TraderId {
    /// Creates a new [`TraderId`] instance.
    /// 创建一个新的 [`TraderId`] 实例。
    ///
    /// Must be correctly formatted with two valid strings either side of a hyphen.
    /// It is expected a trader ID is the abbreviated name of the trader
    /// with an order ID tag number separated by a hyphen.
    /// 必须正确格式化，在连字符的两侧有两个有效的字符串。
    /// 预期交易者 ID 是交易者的缩写名称，用连字符分隔订单 ID 标签号。
    ///
    /// Example: "TESTER-001".
    /// 示例："TESTER-001"。
    ///
    /// The reason for the numerical component of the ID is so that order and position IDs
    /// do not collide with those from another node instance.
    /// ID 的数字组件的原因是为了使订单和持仓 ID 不会与另一个节点实例的 ID 冲突。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if `value` is not a valid string, or does not contain a hyphen '-' separator.
    /// 如果 `value` 不是有效的字符串，或不包含连字符 '-' 分隔符，则返回错误。
    ///
    /// # Notes
    /// # 备注
    ///
    /// PyO3 requires a `Result` type for proper error handling and stacktrace printing in Python.
    /// PyO3 需要 `Result` 类型以便在 Python 中进行适当的错误处理和堆栈跟踪打印。
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let value = value.as_ref();
        check_valid_string_ascii(value, stringify!(value))?;
        check_string_contains(value, "-", stringify!(value))?;
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`TraderId`] instance.
    /// 创建一个新的 [`TraderId`] 实例。
    ///
    /// # Panics
    /// # 可能 panic 的情况
    ///
    /// Panics if `value` is not a valid string, or does not contain a hyphen '-' separator.
    /// 如果 `value` 不是有效的字符串，或不包含连字符 '-' 分隔符，则会 panic。
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self::new_checked(value).expect(FAILED)
    }

    /// Sets the inner identifier value.
    /// 设置内部标识符值。
    #[cfg_attr(not(feature = "python"), allow(dead_code))]
    pub(crate) fn set_inner(&mut self, value: &str) {
        self.0 = Ustr::from(value);
    }

    /// Returns the inner identifier value.
    /// 返回内部标识符值。
    #[must_use]
    pub fn inner(&self) -> Ustr {
        self.0
    }

    /// Returns the inner identifier value as a string slice.
    /// 将内部标识符值作为字符串切片返回。
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the numerical tag portion of the trader ID.
    ///
    /// # Panics
    ///
    /// Panics if the internal ID string does not contain a '-' separator.
    #[must_use]
    pub fn get_tag(&self) -> &str {
        // SAFETY: Unwrap safe as value previously validated
        self.0.split('-').next_back().unwrap()
    }
}

impl Debug for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for TraderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::identifiers::{stubs::*, trader_id::TraderId};

    #[rstest]
    fn test_string_reprs(trader_id: TraderId) {
        assert_eq!(trader_id.as_str(), "TRADER-001");
        assert_eq!(format!("{trader_id}"), "TRADER-001");
    }

    #[rstest]
    fn test_get_tag(trader_id: TraderId) {
        assert_eq!(trader_id.get_tag(), "001");
    }
}
