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

//! Represents a valid strategy ID.
//! 表示有效的策略 ID。

use std::fmt::{Debug, Display, Formatter};

use nautilus_core::correctness::{FAILED, check_string_contains, check_valid_string_ascii};
use ustr::Ustr;

/// The identifier for all 'external' strategy IDs (not local to this system instance).
/// 所有"外部"策略 ID 的标识符（不在此系统实例本地）。
const EXTERNAL_STRATEGY_ID: &str = "EXTERNAL";

/// Represents a valid strategy ID.
/// 表示有效的策略 ID。
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.model")
)]
pub struct StrategyId(Ustr);

impl StrategyId {
    /// Creates a new [`StrategyId`] instance.
    /// 创建一个新的 [`StrategyId`] 实例。
    ///
    /// Must be correctly formatted with two valid strings either side of a hyphen.
    /// It is expected a strategy ID is the class name of the strategy,
    /// with an order ID tag number separated by a hyphen.
    /// 必须正确格式化，在连字符的两侧有两个有效的字符串。
    /// 预期策略 ID 是策略的类名，用连字符分隔订单 ID 标签号。
    ///
    /// Example: "EMACross-001".
    /// 示例："EMACross-001"。
    ///
    /// The reason for the numerical component of the ID is so that order and position IDs
    /// do not collide with those from another strategy within the node instance.
    /// ID 的数字组件的原因是为了使订单和持仓 ID 不会与节点实例中另一个策略的 ID 冲突。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if `value` is not a valid strategy format or missing '-' separator.
    /// 如果 `value` 不是有效的策略格式或缺少 '-' 分隔符，则返回错误。
    ///
    /// # Panics
    /// # 可能 panic 的情况
    ///
    /// Panics if `value` is not a valid string, or does not contain a hyphen '-' separator.
    /// 如果 `value` 不是有效的字符串，或不包含连字符 '-' 分隔符，则会 panic。
    pub fn new_checked<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let value = value.as_ref();
        check_valid_string_ascii(value, stringify!(value))?;
        if value != EXTERNAL_STRATEGY_ID {
            check_string_contains(value, "-", stringify!(value))?;
        }
        Ok(Self(Ustr::from(value)))
    }

    /// Creates a new [`StrategyId`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid string.
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self::new_checked(value).expect(FAILED)
    }

    /// Sets the inner identifier value.
    #[cfg_attr(not(feature = "python"), allow(dead_code))]
    pub(crate) fn set_inner(&mut self, value: &str) {
        self.0 = Ustr::from(value);
    }

    /// Returns the inner identifier value.
    #[must_use]
    pub fn inner(&self) -> Ustr {
        self.0
    }

    /// Returns the inner identifier value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn external() -> Self {
        // SAFETY:: Constant value is safe
        Self::new(EXTERNAL_STRATEGY_ID)
    }

    #[must_use]
    pub fn is_external(&self) -> bool {
        self.0 == EXTERNAL_STRATEGY_ID
    }

    /// Returns the numerical tag portion of the strategy ID.
    ///
    /// # Panics
    ///
    /// Panics if the internal ID does not contain a '-' separator.
    #[must_use]
    pub fn get_tag(&self) -> &str {
        // SAFETY: Unwrap safe as value previously validated
        self.0.split('-').next_back().unwrap()
    }
}

impl Debug for StrategyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for StrategyId {
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

    use super::StrategyId;
    use crate::identifiers::stubs::*;

    #[rstest]
    fn test_string_reprs(strategy_id_ema_cross: StrategyId) {
        assert_eq!(strategy_id_ema_cross.as_str(), "EMACross-001");
        assert_eq!(format!("{strategy_id_ema_cross}"), "EMACross-001");
    }

    #[rstest]
    fn test_get_external() {
        assert_eq!(StrategyId::external().as_str(), "EXTERNAL");
    }

    #[rstest]
    fn test_is_external() {
        assert!(StrategyId::external().is_external());
    }

    #[rstest]
    fn test_get_tag(strategy_id_ema_cross: StrategyId) {
        assert_eq!(strategy_id_ema_cross.get_tag(), "001");
    }
}
