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

use std::any::Any;

use nautilus_infrastructure::sql::pg::PostgresConnectOptions;
use nautilus_model::{
    defi::{Chain, DexType, SharedChain},
    identifiers::{AccountId, TraderId},
};
use nautilus_system::ClientConfig;

/// Defines filtering criteria for the DEX pool universe that the data client will operate on.
/// 定义数据客户端将操作的 DEX 池集合的过滤条件。
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.blockchain")
)]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass(module = "nautilus_trader.adapters.blockchain")
)]
pub struct DexPoolFilters {
    /// Whether to exclude pools containing tokens with empty name or symbol fields.
    /// 是否排除包含具有空名称或符号字段的代币的池。
    pub remove_pools_with_empty_erc20fields: bool,
}

impl DexPoolFilters {
    /// Creates a new [`DexPoolFilters`] instance.
    /// 创建一个新的 [`DexPoolFilters`] 实例。
    #[must_use]
    pub fn new(remove_pools_with_empty_erc20fields: Option<bool>) -> Self {
        Self {
            remove_pools_with_empty_erc20fields: remove_pools_with_empty_erc20fields
                .unwrap_or(true),
        }
    }
}

impl Default for DexPoolFilters {
    fn default() -> Self {
        Self {
            remove_pools_with_empty_erc20fields: true,
        }
    }
}

/// Configuration for blockchain data clients.
/// 区块链数据客户端的配置。
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.blockchain")
)]
#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass(module = "nautilus_trader.adapters.blockchain")
)]
pub struct BlockchainDataClientConfig {
    /// The blockchain chain configuration.
    /// 区块链链配置。
    pub chain: SharedChain,
    /// List of decentralized exchange IDs to register and sync during connection.
    /// 在连接期间要注册和同步的去中心化交易所 ID 列表。
    pub dex_ids: Vec<DexType>,
    /// Determines if the client should use Hypersync for live data streaming.
    /// 确定客户端是否应使用 Hypersync 进行实时数据流。
    pub use_hypersync_for_live_data: bool,
    /// The HTTP URL for the blockchain RPC endpoint.
    /// 区块链 RPC 端点的 HTTP URL。
    pub http_rpc_url: String,
    /// The maximum number of RPC requests allowed per second.
    /// 每秒允许的最大 RPC 请求数。
    pub rpc_requests_per_second: Option<u32>,
    /// The maximum number of Multicall calls per one RPC request.
    /// 每个 RPC 请求的最大 Multicall 调用数。
    pub multicall_calls_per_rpc_request: u32,
    /// The WebSocket secure URL for the blockchain RPC endpoint.
    /// 区块链 RPC 端点的 WebSocket 安全 URL。
    pub wss_rpc_url: Option<String>,
    /// Optional HTTP proxy URL for RPC requests.
    /// RPC 请求的可选 HTTP 代理 URL。
    pub http_proxy_url: Option<String>,
    /// Optional WebSocket proxy URL for RPC connections.
    /// RPC 连接的可选 WebSocket 代理 URL。
    ///
    /// Note: WebSocket proxy support is not yet implemented. This field is reserved
    /// for future functionality. Use `http_proxy_url` for REST API proxy support.
    /// 注意：WebSocket 代理支持尚未实现。此字段保留用于未来功能。
    /// 对于 REST API 代理支持，请使用 `http_proxy_url`。
    pub ws_proxy_url: Option<String>,
    /// The block from which to sync historical data.
    /// 从哪个区块开始同步历史数据。
    pub from_block: Option<u64>,
    /// Filtering criteria that define which DEX pools to include in the data universe.
    /// 定义要在数据集合中包含哪些 DEX 池的过滤条件。
    pub pool_filters: DexPoolFilters,
    /// Optional configuration for data client's Postgres cache database
    /// 数据客户端的 Postgres 缓存数据库的可选配置
    pub postgres_cache_database_config: Option<PostgresConnectOptions>,
}

impl BlockchainDataClientConfig {
    /// Creates a new [`BlockchainDataClientConfig`] instance.
    /// 创建一个新的 [`BlockchainDataClientConfig`] 实例。
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        chain: SharedChain,
        dex_ids: Vec<DexType>,
        http_rpc_url: String,
        rpc_requests_per_second: Option<u32>,
        multicall_calls_per_rpc_request: Option<u32>,
        wss_rpc_url: Option<String>,
        use_hypersync_for_live_data: bool,
        from_block: Option<u64>,
        pools_filters: Option<DexPoolFilters>,
        postgres_cache_database_config: Option<PostgresConnectOptions>,
    ) -> Self {
        Self {
            chain,
            dex_ids,
            use_hypersync_for_live_data,
            http_rpc_url,
            rpc_requests_per_second,
            multicall_calls_per_rpc_request: multicall_calls_per_rpc_request.unwrap_or(200),
            wss_rpc_url,
            http_proxy_url: None,
            ws_proxy_url: None,
            from_block,
            pool_filters: pools_filters.unwrap_or_default(),
            postgres_cache_database_config,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockchainExecutionClientConfig {
    /// The trader ID for the client.
    pub trader_id: TraderId,
    /// The account ID for the client.
    pub client_id: AccountId,
    /// The blockchain chain configuration.
    pub chain: Chain,
    /// The wallet address of the execution client.
    pub wallet_address: String,
    /// Token universe: set of ERC-20 token addresses to monitor for balance tracking.
    pub tokens: Option<Vec<String>>,
    /// The HTTP URL for the blockchain RPC endpoint.
    pub http_rpc_url: String,
    /// The maximum number of RPC requests allowed per second.
    pub rpc_requests_per_second: Option<u32>,
}

impl BlockchainExecutionClientConfig {
    pub fn new(
        trader_id: TraderId,
        client_id: AccountId,
        chain: Chain,
        wallet_address: String,
        tokens: Option<Vec<String>>,
        http_rpc_url: String,
        rpc_requests_per_second: Option<u32>,
    ) -> Self {
        Self {
            trader_id,
            client_id,
            chain,
            wallet_address,
            tokens,
            http_rpc_url,
            rpc_requests_per_second,
        }
    }
}

impl ClientConfig for BlockchainExecutionClientConfig {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
