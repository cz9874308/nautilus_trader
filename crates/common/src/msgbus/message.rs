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

use std::fmt::Display;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use ustr::Ustr;

use super::switchboard::CLOSE_TOPIC;

/// Represents a bus message including a topic and serialized payload.
/// 表示包含主题和序列化有效负载的总线消息。
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.common")
)]
pub struct BusMessage {
    /// The topic to publish the message on.
    /// 发布消息的主题。
    pub topic: Ustr,
    /// The serialized payload for the message.
    /// 消息的序列化有效负载。
    pub payload: Bytes,
}

impl BusMessage {
    /// Creates a new [`BusMessage`] instance.
    /// 创建一个新的 [`BusMessage`] 实例。
    pub fn new(topic: Ustr, payload: Bytes) -> Self {
        debug_assert!(!topic.is_empty());
        Self { topic, payload }
    }

    /// Creates a new [`BusMessage`] instance with a string-like topic.
    /// 使用类似字符串的主题创建新的 [`BusMessage`] 实例。
    ///
    /// This is a convenience constructor that converts any string-like type
    /// (implementing `AsRef<str>`) into the required `Ustr` type.
    /// 这是一个便捷构造函数，将任何类似字符串的类型（实现 `AsRef<str>`）转换为所需的 `Ustr` 类型。
    pub fn with_str_topic<T: AsRef<str>>(topic: T, payload: Bytes) -> Self {
        Self::new(Ustr::from(topic.as_ref()), payload)
    }

    /// Creates a new [`BusMessage`] instance with the `CLOSE` topic and empty payload.
    /// 使用 `CLOSE` 主题和空有效负载创建新的 [`BusMessage`] 实例。
    pub fn new_close() -> Self {
        Self::with_str_topic(CLOSE_TOPIC, Bytes::new())
    }
}

impl Display for BusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.topic,
            String::from_utf8_lossy(&self.payload)
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("test/topic", "payload data")]
    #[case("events/trading", "Another payload")]
    fn test_with_str_topic_str(#[case] topic: &str, #[case] payload_str: &str) {
        let payload = Bytes::from(payload_str.to_string());

        let message = BusMessage::with_str_topic(topic, payload.clone());

        assert_eq!(message.topic.as_str(), topic);
        assert_eq!(message.payload, payload);
    }

    #[rstest]
    fn test_with_str_topic_string() {
        let topic_string = String::from("orders/new");
        let payload = Bytes::from("order payload data");

        let message = BusMessage::with_str_topic(topic_string.clone(), payload.clone());

        assert_eq!(message.topic.as_str(), topic_string);
        assert_eq!(message.payload, payload);
    }

    #[rstest]
    fn test_new_close() {
        let message = BusMessage::new_close();

        assert_eq!(message.topic.as_str(), "CLOSE");
        assert!(message.payload.is_empty());
    }
}
