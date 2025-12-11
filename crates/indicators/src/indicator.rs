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

//! A common `Indicator` trait.
//! 通用 `Indicator` trait。

use std::fmt::Debug;

use nautilus_model::{
    data::{Bar, OrderBookDelta, OrderBookDeltas, OrderBookDepth10, QuoteTick, TradeTick},
    orderbook::OrderBook,
};

const IMPL_ERR: &str = "is not implemented for";

/// Common trait for technical analysis indicators.
/// 技术分析指标的通用 trait。
#[allow(unused_variables)]
pub trait Indicator {
    /// Returns the name of the indicator.
    /// 返回指标的名称。
    fn name(&self) -> String;

    /// Returns `true` if the indicator has inputs.
    /// 如果指标有输入，则返回 `true`。
    fn has_inputs(&self) -> bool;

    /// Returns `true` if the indicator is initialized.
    /// 如果指标已初始化，则返回 `true`。
    fn initialized(&self) -> bool;

    /// Handles an order book delta update.
    /// 处理订单簿增量更新。
    fn handle_delta(&mut self, delta: &OrderBookDelta) {
        panic!("`handle_delta` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles order book deltas updates.
    /// 处理订单簿增量更新。
    fn handle_deltas(&mut self, deltas: &OrderBookDeltas) {
        panic!("`handle_deltas` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles an order book depth update.
    /// 处理订单簿深度更新。
    fn handle_depth(&mut self, depth: &OrderBookDepth10) {
        panic!("`handle_depth` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles an order book update.
    /// 处理订单簿更新。
    fn handle_book(&mut self, book: &OrderBook) {
        panic!("`handle_book_mbo` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles a quote tick update.
    /// 处理报价 tick 更新。
    fn handle_quote(&mut self, quote: &QuoteTick) {
        panic!("`handle_quote_tick` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles a trade tick update.
    /// 处理交易 tick 更新。
    fn handle_trade(&mut self, trade: &TradeTick) {
        panic!("`handle_trade_tick` {IMPL_ERR} `{}`", self.name());
    }

    /// Handles a bar update.
    /// 处理 K 线更新。
    fn handle_bar(&mut self, bar: &Bar) {
        panic!("`handle_bar` {IMPL_ERR} `{}`", self.name());
    }

    /// Resets the indicator to its initial state.
    /// 将指标重置为其初始状态。
    fn reset(&mut self);
}

/// Trait for moving average indicators.
/// 移动平均指标的 trait。
pub trait MovingAverage: Indicator {
    /// Returns the current indicator value.
    /// 返回当前指标值。
    fn value(&self) -> f64;
    /// Returns the count of processed inputs.
    /// 返回已处理输入的数量。
    fn count(&self) -> usize;
    /// Updates the indicator with a raw value.
    /// 使用原始值更新指标。
    fn update_raw(&mut self, value: f64);
}

impl Debug for dyn Indicator + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implement custom formatting for the Indicator trait object
        write!(f, "Indicator {{ ... }}")
    }
}

impl Debug for dyn MovingAverage + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implement custom formatting for the Indicator trait object
        write!(f, "MovingAverage()")
    }
}
