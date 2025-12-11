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

use std::{fmt::Debug, time::Duration};

use nautilus_common::{
    cache::CacheConfig, enums::Environment, logging::logger::LoggerConfig,
    msgbus::database::MessageBusConfig,
};
use nautilus_core::UUID4;
use nautilus_data::engine::config::DataEngineConfig;
use nautilus_execution::engine::config::ExecutionEngineConfig;
use nautilus_model::identifiers::TraderId;
use nautilus_persistence::config::StreamingConfig;
use nautilus_portfolio::config::PortfolioConfig;
use nautilus_risk::engine::config::RiskEngineConfig;

/// Configuration trait for a `NautilusKernel` core system instance.
/// `NautilusKernel` 核心系统实例的配置 trait。
pub trait NautilusKernelConfig: Debug {
    /// Returns the kernel environment context.
    /// 返回内核环境上下文。
    fn environment(&self) -> Environment;
    /// Returns the trader ID for the node.
    /// 返回节点的交易者 ID。
    fn trader_id(&self) -> TraderId;
    /// Returns if trading strategy state should be loaded from the database on start.
    /// 返回是否应在启动时从数据库加载交易策略状态。
    fn load_state(&self) -> bool;
    /// Returns if trading strategy state should be saved to the database on stop.
    /// 返回是否应在停止时将交易策略状态保存到数据库。
    fn save_state(&self) -> bool;
    /// Returns the logging configuration for the kernel.
    /// 返回内核的日志记录配置。
    fn logging(&self) -> LoggerConfig;
    /// Returns the unique instance identifier for the kernel.
    /// 返回内核的唯一实例标识符。
    fn instance_id(&self) -> Option<UUID4>;
    /// Returns the timeout for all clients to connect and initialize.
    /// 返回所有客户端连接和初始化的超时时间。
    fn timeout_connection(&self) -> Duration;
    /// Returns the timeout for execution state to reconcile.
    /// 返回执行状态对账的超时时间。
    fn timeout_reconciliation(&self) -> Duration;
    /// Returns the timeout for portfolio to initialize margins and unrealized pnls.
    /// 返回投资组合初始化保证金和未实现盈亏的超时时间。
    fn timeout_portfolio(&self) -> Duration;
    /// Returns the timeout for all engine clients to disconnect.
    /// 返回所有引擎客户端断开的超时时间。
    fn timeout_disconnection(&self) -> Duration;
    /// Returns the timeout after stopping the node to await residual events before final shutdown.
    /// 返回停止节点后等待剩余事件最终关闭的超时时间。
    fn delay_post_stop(&self) -> Duration;
    /// Returns the timeout to await pending tasks cancellation during shutdown.
    /// 返回在关闭期间等待待处理任务取消的超时时间。
    fn timeout_shutdown(&self) -> Duration;
    /// Returns the cache configuration.
    /// 返回缓存配置。
    fn cache(&self) -> Option<CacheConfig>;
    /// Returns the message bus configuration.
    /// 返回消息总线配置。
    fn msgbus(&self) -> Option<MessageBusConfig>;
    /// Returns the data engine configuration.
    /// 返回数据引擎配置。
    fn data_engine(&self) -> Option<DataEngineConfig>;
    /// Returns the risk engine configuration.
    /// 返回风险引擎配置。
    fn risk_engine(&self) -> Option<RiskEngineConfig>;
    /// Returns the execution engine configuration.
    /// 返回执行引擎配置。
    fn exec_engine(&self) -> Option<ExecutionEngineConfig>;
    /// Returns the portfolio configuration.
    /// 返回投资组合配置。
    fn portfolio(&self) -> Option<PortfolioConfig>;
    /// Returns the configuration for streaming to feather files.
    /// 返回流式传输到 feather 文件的配置。
    fn streaming(&self) -> Option<StreamingConfig>;
}

/// Basic implementation of `NautilusKernelConfig` for builder and testing.
/// 用于构建器和测试的 `NautilusKernelConfig` 的基本实现。
#[derive(Debug, Clone)]
pub struct KernelConfig {
    /// The kernel environment context.
    /// 内核环境上下文。
    pub environment: Environment,
    /// The trader ID for the node (must be a name and ID tag separated by a hyphen).
    /// 节点的交易者 ID（必须是名称和 ID 标签，用连字符分隔）。
    pub trader_id: TraderId,
    /// If trading strategy state should be loaded from the database on start.
    /// 是否应在启动时从数据库加载交易策略状态。
    pub load_state: bool,
    /// If trading strategy state should be saved to the database on stop.
    /// 是否应在停止时将交易策略状态保存到数据库。
    pub save_state: bool,
    /// The logging configuration for the kernel.
    /// 内核的日志记录配置。
    pub logging: LoggerConfig,
    /// The unique instance identifier for the kernel
    /// 内核的唯一实例标识符
    pub instance_id: Option<UUID4>,
    /// The timeout for all clients to connect and initialize.
    /// 所有客户端连接和初始化的超时时间。
    pub timeout_connection: Duration,
    /// The timeout for execution state to reconcile.
    /// 执行状态对账的超时时间。
    pub timeout_reconciliation: Duration,
    /// The timeout for portfolio to initialize margins and unrealized pnls.
    /// 投资组合初始化保证金和未实现盈亏的超时时间。
    pub timeout_portfolio: Duration,
    /// The timeout for all engine clients to disconnect.
    /// 所有引擎客户端断开的超时时间。
    pub timeout_disconnection: Duration,
    /// The delay after stopping the node to await residual events before final shutdown.
    /// 停止节点后等待剩余事件最终关闭的延迟。
    pub delay_post_stop: Duration,
    /// The delay to await pending tasks cancellation during shutdown.
    /// 在关闭期间等待待处理任务取消的延迟。
    pub timeout_shutdown: Duration,
    /// The cache configuration.
    /// 缓存配置。
    pub cache: Option<CacheConfig>,
    /// The message bus configuration.
    /// 消息总线配置。
    pub msgbus: Option<MessageBusConfig>,
    /// The data engine configuration.
    /// 数据引擎配置。
    pub data_engine: Option<DataEngineConfig>,
    /// The risk engine configuration.
    /// 风险引擎配置。
    pub risk_engine: Option<RiskEngineConfig>,
    /// The execution engine configuration.
    /// 执行引擎配置。
    pub exec_engine: Option<ExecutionEngineConfig>,
    /// The portfolio configuration.
    /// 投资组合配置。
    pub portfolio: Option<PortfolioConfig>,
    /// The configuration for streaming to feather files.
    /// 流式传输到 feather 文件的配置。
    pub streaming: Option<StreamingConfig>,
}

impl NautilusKernelConfig for KernelConfig {
    fn environment(&self) -> Environment {
        self.environment
    }

    fn trader_id(&self) -> TraderId {
        self.trader_id
    }

    fn load_state(&self) -> bool {
        self.load_state
    }

    fn save_state(&self) -> bool {
        self.save_state
    }

    fn logging(&self) -> LoggerConfig {
        self.logging.clone()
    }

    fn instance_id(&self) -> Option<UUID4> {
        self.instance_id
    }

    fn timeout_connection(&self) -> Duration {
        self.timeout_connection
    }

    fn timeout_reconciliation(&self) -> Duration {
        self.timeout_reconciliation
    }

    fn timeout_portfolio(&self) -> Duration {
        self.timeout_portfolio
    }

    fn timeout_disconnection(&self) -> Duration {
        self.timeout_disconnection
    }

    fn delay_post_stop(&self) -> Duration {
        self.delay_post_stop
    }

    fn timeout_shutdown(&self) -> Duration {
        self.timeout_shutdown
    }

    fn cache(&self) -> Option<CacheConfig> {
        self.cache.clone()
    }

    fn msgbus(&self) -> Option<MessageBusConfig> {
        self.msgbus.clone()
    }

    fn data_engine(&self) -> Option<DataEngineConfig> {
        self.data_engine.clone()
    }

    fn risk_engine(&self) -> Option<RiskEngineConfig> {
        self.risk_engine.clone()
    }

    fn exec_engine(&self) -> Option<ExecutionEngineConfig> {
        self.exec_engine.clone()
    }

    fn portfolio(&self) -> Option<PortfolioConfig> {
        self.portfolio.clone()
    }

    fn streaming(&self) -> Option<StreamingConfig> {
        self.streaming.clone()
    }
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Backtest,
            trader_id: TraderId::default(),
            load_state: false,
            save_state: false,
            logging: LoggerConfig::default(),
            instance_id: None,
            timeout_connection: Duration::from_secs(60),
            timeout_reconciliation: Duration::from_secs(30),
            timeout_portfolio: Duration::from_secs(10),
            timeout_disconnection: Duration::from_secs(10),
            delay_post_stop: Duration::from_secs(10),
            timeout_shutdown: Duration::from_secs(5),
            cache: None,
            msgbus: None,
            data_engine: None,
            risk_engine: None,
            exec_engine: None,
            portfolio: None,
            streaming: None,
        }
    }
}
