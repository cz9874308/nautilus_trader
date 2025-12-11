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

use serde::{Deserialize, Serialize};

/// Configuration for `Portfolio` instances.
/// `Portfolio` 实例的配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConfig {
    /// The type of prices used for portfolio calculations, such as unrealized PnLs.
    /// If false (default), uses quote prices if available; otherwise, last trade prices
    /// (or falls back to bar prices if `bar_updates` is true).
    /// If true, uses mark prices.
    /// 用于投资组合计算的价格类型，例如未实现盈亏。
    /// 如果为 false（默认），则在可用时使用报价价格；否则使用最后交易价格
    /// （如果 `bar_updates` 为 true，则回退到 K 线价格）。
    /// 如果为 true，则使用标记价格。
    #[serde(default)]
    pub use_mark_prices: bool,
    /// The type of exchange rates used for portfolio calculations.
    /// If false (default), uses quote prices.
    /// If true, uses mark prices.
    /// 用于投资组合计算的汇率类型。
    /// 如果为 false（默认），则使用报价价格。
    /// 如果为 true，则使用标记价格。
    #[serde(default)]
    pub use_mark_xrates: bool,
    /// If external bars should be considered for updating unrealized PnLs.
    /// 是否应考虑外部 K 线来更新未实现盈亏。
    #[serde(default = "default_true")]
    pub bar_updates: bool,
    /// If calculations should be converted into each account's base currency.
    /// This setting is only effective for accounts with a specified base currency.
    /// 是否应将计算转换为每个账户的基础货币。
    /// 此设置仅对具有指定基础货币的账户有效。
    #[serde(default = "default_true")]
    pub convert_to_account_base_currency: bool,
    /// The minimum interval (milliseconds) between logging account state events for the same account.
    /// When set, account state updates will only be logged if this much time has passed since the last log.
    /// Useful for HFT deployments to prevent excessive logging when account states change rapidly.
    /// 为同一账户记录账户状态事件之间的最小间隔（毫秒）。
    /// 设置后，只有在自上次记录以来经过这么多时间后，才会记录账户状态更新。
    /// 对于 HFT 部署很有用，可在账户状态快速变化时防止过度日志记录。
    #[serde(default)]
    pub min_account_state_logging_interval_ms: Option<u64>,
    /// If debug mode is active (will provide extra debug logging).
    /// 如果调试模式处于活动状态（将提供额外的调试日志记录）。
    #[serde(default)]
    pub debug: bool,
}

const fn default_true() -> bool {
    true
}

impl Default for PortfolioConfig {
    fn default() -> Self {
        Self {
            use_mark_prices: false,
            use_mark_xrates: false,
            bar_updates: true,
            convert_to_account_base_currency: true,
            min_account_state_logging_interval_ms: None,
            debug: false,
        }
    }
}
