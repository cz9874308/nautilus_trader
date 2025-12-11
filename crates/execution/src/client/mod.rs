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

//! Execution client implementations for trading venue connectivity.
//! 用于交易场所连接的执行客户端实现。

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use async_trait::async_trait;
use nautilus_common::messages::execution::{
    BatchCancelOrders, CancelAllOrders, CancelOrder, ModifyOrder, QueryAccount, QueryOrder,
    SubmitOrder, SubmitOrderList,
};
use nautilus_core::UnixNanos;
use nautilus_model::{
    accounts::AccountAny,
    enums::OmsType,
    identifiers::{AccountId, ClientId, Venue},
    types::{AccountBalance, MarginBalance},
};

pub mod base;

/// Defines the interface for an execution client managing order operations.
/// 定义用于管理订单操作的执行客户端接口。
///
/// # Thread safety
/// # 线程安全
///
/// Client instances are not intended to be sent across threads. The `?Send` bound
/// allows implementations to hold non-Send state for any Python interop.
/// 客户端实例不打算跨线程发送。`?Send` 绑定允许实现为任何 Python 互操作持有非 Send 状态。
#[async_trait(?Send)]
pub trait ExecutionClient {
    /// Returns `true` if the client is currently connected.
    /// 如果客户端当前已连接，则返回 `true`。
    fn is_connected(&self) -> bool;
    /// Returns the unique identifier for this execution client.
    /// 返回此执行客户端的唯一标识符。
    fn client_id(&self) -> ClientId;
    /// Returns the account ID associated with this client.
    /// 返回与此客户端关联的账户 ID。
    fn account_id(&self) -> AccountId;
    /// Returns the venue this client is connected to.
    /// 返回此客户端连接到的场所。
    fn venue(&self) -> Venue;
    /// Returns the order management system type.
    /// 返回订单管理系统类型。
    fn oms_type(&self) -> OmsType;
    /// Returns the account associated with this client, if available.
    /// 返回与此客户端关联的账户（如果可用）。
    fn get_account(&self) -> Option<AccountAny>;

    /// Generates and publishes the account state event.
    /// 生成并发布账户状态事件。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if generating the account state fails.
    /// 如果生成账户状态失败，则返回错误。
    fn generate_account_state(
        &self,
        balances: Vec<AccountBalance>,
        margins: Vec<MarginBalance>,
        reported: bool,
        ts_event: UnixNanos,
    ) -> anyhow::Result<()>;

    /// Starts the execution client.
    /// 启动执行客户端。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the client fails to start.
    /// 如果客户端启动失败，则返回错误。
    fn start(&mut self) -> anyhow::Result<()>;

    /// Stops the execution client.
    /// 停止执行客户端。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the client fails to stop.
    /// 如果客户端停止失败，则返回错误。
    fn stop(&mut self) -> anyhow::Result<()>;

    /// Connects the client to the execution venue.
    /// 将客户端连接到执行场所。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if connection fails.
    /// 如果连接失败，则返回错误。
    async fn connect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Disconnects the client from the execution venue.
    /// 将客户端与执行场所断开连接。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if disconnection fails.
    /// 如果断开连接失败，则返回错误。
    async fn disconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Submits a single order command to the execution venue.
    /// 向执行场所提交单个订单命令。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if submission fails.
    fn submit_order(&self, cmd: &SubmitOrder) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Submits a list of orders to the execution venue.
    ///
    /// # Errors
    ///
    /// Returns an error if submission fails.
    fn submit_order_list(&self, cmd: &SubmitOrderList) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Modifies an existing order.
    ///
    /// # Errors
    ///
    /// Returns an error if modification fails.
    fn modify_order(&self, cmd: &ModifyOrder) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Cancels a specific order.
    ///
    /// # Errors
    ///
    /// Returns an error if cancellation fails.
    fn cancel_order(&self, cmd: &CancelOrder) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Cancels all orders.
    ///
    /// # Errors
    ///
    /// Returns an error if cancellation fails.
    fn cancel_all_orders(&self, cmd: &CancelAllOrders) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Cancels a batch of orders.
    ///
    /// # Errors
    ///
    /// Returns an error if batch cancellation fails.
    fn batch_cancel_orders(&self, cmd: &BatchCancelOrders) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Queries the status of an account.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    fn query_account(&self, cmd: &QueryAccount) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }

    /// Queries the status of an order.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    fn query_order(&self, cmd: &QueryOrder) -> anyhow::Result<()> {
        log_not_implemented(cmd);
        Ok(())
    }
}

#[inline(always)]
fn log_not_implemented<T: Debug>(cmd: &T) {
    log::warn!("{cmd:?} – handler not implemented");
}

/// Wraps an [`ExecutionClient`], managing its lifecycle and providing access to the client.
pub struct ExecutionClientAdapter {
    pub(crate) client: Box<dyn ExecutionClient>,
    pub client_id: ClientId,
    pub venue: Venue,
    pub account_id: AccountId,
    pub oms_type: OmsType,
}

impl Deref for ExecutionClientAdapter {
    type Target = Box<dyn ExecutionClient>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for ExecutionClientAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl Debug for ExecutionClientAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ExecutionClientAdapter))
            .field("client_id", &self.client_id)
            .field("venue", &self.venue)
            .field("account_id", &self.account_id)
            .field("oms_type", &self.oms_type)
            .finish()
    }
}

impl ExecutionClientAdapter {
    /// Creates a new [`ExecutionClientAdapter`] with the given client.
    #[must_use]
    pub fn new(client: Box<dyn ExecutionClient>) -> Self {
        let client_id = client.client_id();
        let venue = client.venue();
        let account_id = client.account_id();
        let oms_type = client.oms_type();

        Self {
            client,
            client_id,
            venue,
            account_id,
            oms_type,
        }
    }

    /// Connects the execution client to the venue.
    ///
    /// # Errors
    ///
    /// Returns an error if connection fails.
    pub async fn connect(&mut self) -> anyhow::Result<()> {
        self.client.connect().await
    }

    /// Disconnects the execution client from the venue.
    ///
    /// # Errors
    ///
    /// Returns an error if disconnection fails.
    pub async fn disconnect(&mut self) -> anyhow::Result<()> {
        self.client.disconnect().await
    }
}
