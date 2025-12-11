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

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

use ahash::{AHashMap, AHashSet};
use handler::ShareableMessageHandler;
use indexmap::IndexMap;
use matching::is_matching_backtracking;
use nautilus_core::{
    UUID4,
    correctness::{FAILED, check_predicate_true, check_valid_string_utf8},
};
use nautilus_model::identifiers::TraderId;
use serde::{Deserialize, Serialize};
use switchboard::MessagingSwitchboard;
use ustr::Ustr;

use super::{handler, matching, set_message_bus, switchboard};

#[inline(always)]
fn check_fully_qualified_string(value: &Ustr, key: &str) -> anyhow::Result<()> {
    check_predicate_true(
        !value.chars().any(|c| c == '*' || c == '?'),
        &format!("{key} `value` contained invalid characters, was {value}"),
    )
}

/// Pattern is a string pattern for a subscription with special characters for pattern matching.
/// Pattern 是用于订阅的字符串模式，具有用于模式匹配的特殊字符。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pattern;

/// Topic is a fully qualified string for publishing data.
/// Topic 是用于发布数据的完全限定字符串。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Topic;

/// Endpoint is a fully qualified string for sending data.
/// Endpoint 是用于发送数据的完全限定字符串。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Endpoint;

/// A message bus string type. It can be a pattern or a topic.
/// 消息总线字符串类型。它可以是模式或主题。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MStr<T> {
    value: Ustr,
    #[serde(skip)]
    _marker: std::marker::PhantomData<T>,
}

impl<T> Display for MStr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Deref for MStr<T> {
    type Target = Ustr;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> AsRef<str> for MStr<T> {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl MStr<Pattern> {
    /// Create a new pattern from a string.
    /// 从字符串创建新模式。
    pub fn pattern<T: AsRef<str>>(value: T) -> Self {
        let value = Ustr::from(value.as_ref());

        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl From<&str> for MStr<Pattern> {
    fn from(value: &str) -> Self {
        Self::pattern(value)
    }
}

impl From<String> for MStr<Pattern> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for MStr<Pattern> {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<MStr<Topic>> for MStr<Pattern> {
    fn from(value: MStr<Topic>) -> Self {
        Self {
            value: value.value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl MStr<Topic> {
    /// Create a new topic from a fully qualified string.
    /// 从完全限定字符串创建新主题。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the topic has white space or invalid characters.
    /// 如果主题包含空格或无效字符，则返回错误。
    pub fn topic<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let topic = Ustr::from(value.as_ref());
        check_valid_string_utf8(value, stringify!(value))?;
        check_fully_qualified_string(&topic, stringify!(Topic))?;

        Ok(Self {
            value: topic,
            _marker: std::marker::PhantomData,
        })
    }
}

impl From<&str> for MStr<Topic> {
    fn from(value: &str) -> Self {
        Self::topic(value).expect(FAILED)
    }
}

impl From<String> for MStr<Topic> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for MStr<Topic> {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<Ustr> for MStr<Topic> {
    fn from(value: Ustr) -> Self {
        value.as_str().into()
    }
}

impl From<&Ustr> for MStr<Topic> {
    fn from(value: &Ustr) -> Self {
        (*value).into()
    }
}

impl MStr<Endpoint> {
    /// Create a new endpoint from a fully qualified string.
    /// 从完全限定字符串创建新端点。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if the endpoint has white space or invalid characters.
    /// 如果端点包含空格或无效字符，则返回错误。
    pub fn endpoint<T: AsRef<str>>(value: T) -> anyhow::Result<Self> {
        let endpoint = Ustr::from(value.as_ref());
        check_valid_string_utf8(value, stringify!(value))?;
        check_fully_qualified_string(&endpoint, stringify!(Endpoint))?;

        Ok(Self {
            value: endpoint,
            _marker: std::marker::PhantomData,
        })
    }
}

impl From<&str> for MStr<Endpoint> {
    fn from(value: &str) -> Self {
        Self::endpoint(value).expect(FAILED)
    }
}

impl From<String> for MStr<Endpoint> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for MStr<Endpoint> {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<Ustr> for MStr<Endpoint> {
    fn from(value: Ustr) -> Self {
        value.as_str().into()
    }
}

/// Represents a subscription to a particular topic.
/// 表示对特定主题的订阅。
///
/// This is an internal class intended to be used by the message bus to organize
/// topics and their subscribers.
/// 这是一个内部类，旨在供消息总线用于组织主题及其订阅者。
///
#[derive(Clone, Debug)]
pub struct Subscription {
    /// The shareable message handler for the subscription.
    /// 订阅的可共享消息处理器。
    pub handler: ShareableMessageHandler,
    /// Store a copy of the handler ID for faster equality checks.
    /// 存储处理器 ID 的副本以加快相等性检查。
    pub handler_id: Ustr,
    /// The pattern for the subscription.
    /// 订阅的模式。
    pub pattern: MStr<Pattern>,
    /// The priority for the subscription determines the ordering of handlers receiving
    /// messages being processed, higher priority handlers will receive messages before
    /// lower priority handlers.
    /// 订阅的优先级决定了接收正在处理的消息的处理器顺序，
    /// 高优先级处理器将在低优先级处理器之前接收消息。
    pub priority: u8,
}

impl Subscription {
    /// Creates a new [`Subscription`] instance.
    /// 创建一个新的 [`Subscription`] 实例。
    #[must_use]
    pub fn new(
        pattern: MStr<Pattern>,
        handler: ShareableMessageHandler,
        priority: Option<u8>,
    ) -> Self {
        Self {
            handler_id: handler.0.id(),
            pattern,
            handler,
            priority: priority.unwrap_or(0),
        }
    }
}

impl PartialEq<Self> for Subscription {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.handler_id == other.handler_id
    }
}

impl Eq for Subscription {}

impl PartialOrd for Subscription {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Subscription {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.pattern.cmp(&other.pattern))
            .then_with(|| self.handler_id.cmp(&other.handler_id))
    }
}

impl Hash for Subscription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
        self.handler_id.hash(state);
    }
}

/// A generic message bus to facilitate various messaging patterns.
/// 用于促进各种消息传递模式的通用消息总线。
///
/// The bus provides both a producer and consumer API for Pub/Sub, Req/Rep, as
/// well as direct point-to-point messaging to registered endpoints.
/// 总线为 Pub/Sub、Req/Rep 以及到注册端点的直接点对点消息传递提供生产者和消费者 API。
///
/// Pub/Sub wildcard patterns for hierarchical topics are possible:
/// 分层主题的 Pub/Sub 通配符模式是可能的：
///  - `*` asterisk represents one or more characters in a pattern.
///    `*` 星号表示模式中的一个或多个字符。
///  - `?` question mark represents a single character in a pattern.
///    `?` 问号表示模式中的单个字符。
///
/// Given a topic and pattern potentially containing wildcard characters, i.e.
/// `*` and `?`, where `?` can match any single character in the topic, and `*`
/// can match any number of characters including zero characters.
/// 给定一个可能包含通配符字符的主题和模式，即 `*` 和 `?`，
/// 其中 `?` 可以匹配主题中的任何单个字符，`*` 可以匹配任意数量的字符（包括零个字符）。
///
/// The asterisk in a wildcard matches any character zero or more times. For
/// example, `comp*` matches anything beginning with `comp` which means `comp`,
/// `complete`, and `computer` are all matched.
/// 通配符中的星号匹配任何字符零次或多次。例如，`comp*` 匹配任何以 `comp` 开头的内容，
/// 这意味着 `comp`、`complete` 和 `computer` 都匹配。
///
/// A question mark matches a single character once. For example, `c?mp` matches
/// `camp` and `comp`. The question mark can also be used more than once.
/// For example, `c??p` would match both of the above examples and `coop`.
/// 问号匹配单个字符一次。例如，`c?mp` 匹配 `camp` 和 `comp`。
/// 问号也可以多次使用。例如，`c??p` 将匹配上述两个示例和 `coop`。
#[derive(Debug)]
pub struct MessageBus {
    /// The trader ID associated with the message bus.
    /// 与消息总线关联的交易者 ID。
    pub trader_id: TraderId,
    /// The instance ID associated with the message bus.
    /// 与消息总线关联的实例 ID。
    pub instance_id: UUID4,
    /// The name for the message bus.
    /// 消息总线的名称。
    pub name: String,
    /// If the message bus is backed by a database.
    /// 消息总线是否由数据库支持。
    pub has_backing: bool,
    /// The switchboard for built-in endpoints.
    /// 内置端点的交换板。
    pub switchboard: MessagingSwitchboard,
    /// Active subscriptions.
    /// 活动订阅。
    pub subscriptions: AHashSet<Subscription>,
    /// Maps a topic to all the handlers registered for it
    /// this is updated whenever a new subscription is created.
    /// 将主题映射到为其注册的所有处理器，每当创建新订阅时都会更新此映射。
    pub topics: IndexMap<MStr<Topic>, Vec<Subscription>>,
    /// Index of endpoint addresses and their handlers.
    /// 端点地址及其处理器的索引。
    pub endpoints: IndexMap<MStr<Endpoint>, ShareableMessageHandler>,
    /// Index of request correlation IDs and their response handlers.
    /// 请求关联 ID 及其响应处理器的索引。
    pub correlation_index: AHashMap<UUID4, ShareableMessageHandler>,
}

// MessageBus is designed for single-threaded use within each async runtime.
// Thread-local storage ensures each thread gets its own instance, eliminating
// the need for unsafe Send/Sync implementations that were previously required
// for global static storage.

impl MessageBus {
    /// Creates a new [`MessageBus`] instance.
    /// 创建一个新的 [`MessageBus`] 实例。
    #[must_use]
    pub fn new(
        trader_id: TraderId,
        instance_id: UUID4,
        name: Option<String>,
        _config: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            trader_id,
            instance_id,
            name: name.unwrap_or(stringify!(MessageBus).to_owned()),
            switchboard: MessagingSwitchboard::default(),
            subscriptions: AHashSet::new(),
            topics: IndexMap::new(),
            endpoints: IndexMap::new(),
            correlation_index: AHashMap::new(),
            has_backing: false,
        }
    }

    /// Returns the memory address of this instance as a hexadecimal string.
    /// 返回此实例的内存地址作为十六进制字符串。
    #[must_use]
    pub fn mem_address(&self) -> String {
        format!("{self:p}")
    }

    /// Returns the registered endpoint addresses.
    /// 返回已注册的端点地址。
    #[must_use]
    pub fn endpoints(&self) -> Vec<&str> {
        self.endpoints.iter().map(|e| e.0.as_str()).collect()
    }

    /// Returns actively subscribed patterns.
    /// 返回活动订阅的模式。
    #[must_use]
    pub fn patterns(&self) -> Vec<&str> {
        self.subscriptions
            .iter()
            .map(|s| s.pattern.as_str())
            .collect()
    }

    /// Returns whether there are subscribers for the `topic`.
    pub fn has_subscribers<T: AsRef<str>>(&self, topic: T) -> bool {
        self.subscriptions_count(topic) > 0
    }

    /// Returns the count of subscribers for the `topic`.
    ///
    /// # Panics
    ///
    /// Returns an error if the topic is not valid.
    #[must_use]
    pub fn subscriptions_count<T: AsRef<str>>(&self, topic: T) -> usize {
        let topic = MStr::<Topic>::topic(topic).expect(FAILED);
        self.topics
            .get(&topic)
            .map_or_else(|| self.find_topic_matches(topic).len(), |subs| subs.len())
    }

    /// Returns active subscriptions.
    #[must_use]
    pub fn subscriptions(&self) -> Vec<&Subscription> {
        self.subscriptions.iter().collect()
    }

    /// Returns the handler IDs for actively subscribed patterns.
    #[must_use]
    pub fn subscription_handler_ids(&self) -> Vec<&str> {
        self.subscriptions
            .iter()
            .map(|s| s.handler_id.as_str())
            .collect()
    }

    /// Returns whether the endpoint is registered.
    ///
    /// # Panics
    ///
    /// Returns an error if the endpoint is not valid topic string.
    #[must_use]
    pub fn is_registered<T: Into<MStr<Endpoint>>>(&self, endpoint: T) -> bool {
        let endpoint: MStr<Endpoint> = endpoint.into();
        self.endpoints.contains_key(&endpoint)
    }

    /// Returns whether the `handler` is subscribed to the `pattern`.
    #[must_use]
    pub fn is_subscribed<T: AsRef<str>>(
        &self,
        pattern: T,
        handler: ShareableMessageHandler,
    ) -> bool {
        let pattern = MStr::<Pattern>::pattern(pattern);
        let sub = Subscription::new(pattern, handler, None);
        self.subscriptions.contains(&sub)
    }

    /// Close the message bus which will close the sender channel and join the thread.
    ///
    /// # Errors
    ///
    /// This function never returns an error (TBD once backing database added).
    pub const fn close(&self) -> anyhow::Result<()> {
        // TODO: Integrate the backing database
        Ok(())
    }

    /// Returns the handler for the `endpoint`.
    #[must_use]
    pub fn get_endpoint(&self, endpoint: MStr<Endpoint>) -> Option<&ShareableMessageHandler> {
        self.endpoints.get(&endpoint)
    }

    /// Returns the handler for the `correlation_id`.
    #[must_use]
    pub fn get_response_handler(&self, correlation_id: &UUID4) -> Option<&ShareableMessageHandler> {
        self.correlation_index.get(correlation_id)
    }

    /// Finds the subscriptions with pattern matching the `topic`.
    pub(crate) fn find_topic_matches(&self, topic: MStr<Topic>) -> Vec<Subscription> {
        self.subscriptions
            .iter()
            .filter_map(|sub| {
                if is_matching_backtracking(topic, sub.pattern) {
                    Some(sub.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Finds the subscriptions which match the `topic` and caches the
    /// results in the `patterns` map.
    #[must_use]
    pub fn matching_subscriptions<T: Into<MStr<Topic>>>(&mut self, topic: T) -> Vec<Subscription> {
        self.inner_matching_subscriptions(topic.into())
    }

    pub(crate) fn inner_matching_subscriptions(&mut self, topic: MStr<Topic>) -> Vec<Subscription> {
        self.topics.get(&topic).cloned().unwrap_or_else(|| {
            let mut matches = self.find_topic_matches(topic);
            matches.sort();
            self.topics.insert(topic, matches.clone());
            matches
        })
    }

    /// Registers a response handler for a specific correlation ID.
    ///
    /// # Errors
    ///
    /// Returns an error if `handler` is already registered for the `correlation_id`.
    pub fn register_response_handler(
        &mut self,
        correlation_id: &UUID4,
        handler: ShareableMessageHandler,
    ) -> anyhow::Result<()> {
        if self.correlation_index.contains_key(correlation_id) {
            anyhow::bail!("Correlation ID <{correlation_id}> already has a registered handler");
        }

        self.correlation_index.insert(*correlation_id, handler);

        Ok(())
    }
}

/// Data specific functions.
impl MessageBus {
    // /// Send a [`DataRequest`] to an endpoint that must be a data client implementation.
    // pub fn send_data_request(&self, message: DataRequest) {
    //     // TODO: log error
    //     if let Some(client) = self.get_client(&message.client_id, message.venue) {
    //         let _ = client.request(message);
    //     }
    // }
    //
    // /// Send a [`SubscriptionCommand`] to an endpoint that must be a data client implementation.
    // pub fn send_subscription_command(&self, message: SubscriptionCommand) {
    //     if let Some(client) = self.get_client(&message.client_id, message.venue) {
    //         client.through_execute(message);
    //     }
    // }

    /// Registers message bus for the current thread.
    pub fn register_message_bus(self) -> Rc<RefCell<Self>> {
        let msgbus = Rc::new(RefCell::new(self));
        set_message_bus(msgbus.clone());
        msgbus
    }
}

impl Default for MessageBus {
    /// Creates a new default [`MessageBus`] instance.
    fn default() -> Self {
        Self::new(TraderId::from("TRADER-001"), UUID4::new(), None, None)
    }
}
