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

//! Represents a valid account ID.
//! 表示有效的账户 ID。

use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

use nautilus_core::correctness::{FAILED, check_string_contains, check_valid_string_ascii};
use ustr::Ustr;

use super::Venue;

/// Represents a valid account ID.
/// 表示有效的账户 ID。
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.model")
)]
pub struct AccountId(Ustr);

impl AccountId {
    /// Creates a new [`AccountId`] instance with correctness checking.
    /// 创建一个新的 [`AccountId`] 实例，并进行正确性检查。
    ///
    /// Must be correctly formatted with two valid strings either side of a hyphen '-'.
    /// 必须正确格式化，在连字符 '-' 的两侧有两个有效的字符串。
    ///
    /// It is expected an account ID is the name of the issuer with an account number
    /// separated by a hyphen.
    /// 预期账户 ID 是发行者名称和账户号，用连字符分隔。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if:
    /// 在以下情况下返回错误：
    /// - `value` is not a valid string.
    ///   `value` 不是有效的字符串。
    /// - `value` length is greater than 36.
    ///   `value` 长度大于 36。
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

    /// Creates a new [`AccountId`] instance.
    /// 创建一个新的 [`AccountId`] 实例。
    ///
    /// # Panics
    /// # 可能 panic 的情况
    ///
    /// Panics if `value` is not a valid string, or value length is greater than 36.
    /// 如果 `value` 不是有效的字符串，或值长度大于 36，则会 panic。
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
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the account issuer for this identifier.
    ///
    /// # Panics
    ///
    /// Panics if the internal ID does not contain a hyphen separator.
    #[must_use]
    pub fn get_issuer(&self) -> Venue {
        // SAFETY: Account ID is guaranteed to have chars either side of a hyphen
        Venue::from_str_unchecked(self.0.split_once('-').unwrap().0)
    }

    /// Returns the account ID assigned by the issuer.
    ///
    /// # Panics
    ///
    /// Panics if the internal ID does not contain a hyphen separator.
    #[must_use]
    pub fn get_issuers_id(&self) -> &str {
        // SAFETY: Account ID is guaranteed to have chars either side of a hyphen
        self.0.split_once('-').unwrap().1
    }
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for AccountId {
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

    use super::*;
    use crate::identifiers::stubs::*;

    #[rstest]
    #[should_panic]
    fn test_account_id_new_invalid_string() {
        AccountId::new("");
    }

    #[rstest]
    #[should_panic]
    fn test_account_id_new_missing_hyphen() {
        AccountId::new("123456789");
    }

    #[rstest]
    fn test_account_id_fmt() {
        let s = "IB-U123456789";
        let account_id = AccountId::new(s);
        let formatted = format!("{account_id}");
        assert_eq!(formatted, s);
    }

    #[rstest]
    fn test_string_reprs(account_ib: AccountId) {
        assert_eq!(account_ib.as_str(), "IB-1234567890");
    }

    #[rstest]
    fn test_get_issuer(account_ib: AccountId) {
        assert_eq!(account_ib.get_issuer(), Venue::new("IB"));
    }

    #[rstest]
    fn test_get_issuers_id(account_ib: AccountId) {
        assert_eq!(account_ib.get_issuers_id(), "1234567890");
    }
}
