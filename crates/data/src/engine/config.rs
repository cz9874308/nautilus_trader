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

use std::{collections::HashMap, time::Duration};

use nautilus_model::{
    enums::{BarAggregation, BarIntervalType},
    identifiers::ClientId,
};

/// Configuration for `DataEngine` instances.
/// `DataEngine` 实例的配置。
#[derive(Clone, Debug)]
pub struct DataEngineConfig {
    /// If time bar aggregators will build and emit bars with no new market updates.
    /// 时间 K 线聚合器是否在没有新市场更新的情况下构建和发出 K 线。
    pub time_bars_build_with_no_updates: bool,
    /// If time bar aggregators will timestamp `ts_event` on bar close.
    /// If False, then will timestamp on bar open.
    /// 时间 K 线聚合器是否在 K 线收盘时标记 `ts_event` 时间戳。
    /// 如果为 False，则将在 K 线开盘时标记时间戳。
    pub time_bars_timestamp_on_close: bool,
    /// If time bar aggregators will skip emitting a bar if the aggregation starts mid-interval.
    /// 如果聚合在间隔中间开始，时间 K 线聚合器是否跳过发出 K 线。
    pub time_bars_skip_first_non_full_bar: bool,
    /// Determines the type of interval used for time aggregation.
    /// 确定用于时间聚合的间隔类型。
    /// - `LeftOpen`: start time is excluded and end time is included (default).
    ///   `LeftOpen`：开始时间被排除，结束时间被包含（默认）。
    /// - `RightOpen`: start time is included and end time is excluded.
    ///   `RightOpen`：开始时间被包含，结束时间被排除。
    pub time_bars_interval_type: BarIntervalType,
    /// A dictionary mapping time bar aggregations to their origin time offsets.
    /// 将时间 K 线聚合映射到其起始时间偏移的字典。
    pub time_bars_origins: HashMap<BarAggregation, Duration>,
    /// If data objects timestamp sequencing will be validated and handled.
    /// 是否将验证和处理数据对象的时间戳序列。
    pub validate_data_sequence: bool,
    /// If order book deltas should be buffered until the `F_LAST` flag is set for a delta.
    /// 是否应缓冲订单簿增量，直到为增量设置 `F_LAST` 标志。
    pub buffer_deltas: bool,
    /// The client IDs declared for external stream processing.
    /// The data engine will not attempt to send data commands to these client IDs.
    /// 为外部流处理声明的客户端 ID。
    /// 数据引擎不会尝试向这些客户端 ID 发送数据命令。
    pub external_clients: Option<Vec<ClientId>>,
    /// If debug mode is active (will provide extra debug logging).
    /// 如果调试模式处于活动状态（将提供额外的调试日志记录）。
    pub debug: bool,
}

impl DataEngineConfig {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        time_bars_build_with_no_updates: bool,
        time_bars_timestamp_on_close: bool,
        time_bars_interval_type: BarIntervalType,
        time_bars_skip_first_non_full_bar: bool,
        time_bars_origins: HashMap<BarAggregation, Duration>,
        validate_data_sequence: bool,
        buffer_deltas: bool,
        external_clients: Option<Vec<ClientId>>,
        debug: bool,
    ) -> Self {
        Self {
            time_bars_build_with_no_updates,
            time_bars_timestamp_on_close,
            time_bars_skip_first_non_full_bar,
            time_bars_interval_type,
            time_bars_origins,
            validate_data_sequence,
            buffer_deltas,
            external_clients,
            debug,
        }
    }
}

impl Default for DataEngineConfig {
    fn default() -> Self {
        Self {
            time_bars_build_with_no_updates: true,
            time_bars_timestamp_on_close: true,
            time_bars_interval_type: BarIntervalType::LeftOpen,
            validate_data_sequence: false,
            buffer_deltas: false,
            external_clients: None,
            debug: false,
            time_bars_skip_first_non_full_bar: false,
            time_bars_origins: HashMap::new(),
        }
    }
}
