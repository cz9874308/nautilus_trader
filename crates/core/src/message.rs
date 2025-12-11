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

//! Common message types.
//! 通用消息类型。

use crate::{UUID4, UnixNanos};

/// Represents different types of messages in the system.
/// 表示系统中的不同类型的消息。
#[derive(Debug, Clone)]
pub enum Message {
    /// A command message with an identifier and initialization timestamp.
    /// 带有标识符和初始化时间戳的命令消息。
    Command {
        /// The unique identifier for this command.
        /// 此命令的唯一标识符。
        id: UUID4,
        /// The initialization timestamp.
        /// 初始化时间戳。
        ts_init: UnixNanos,
    },
    /// A document message with an identifier and initialization timestamp.
    /// 带有标识符和初始化时间戳的文档消息。
    Document {
        /// The unique identifier for this document.
        /// 此文档的唯一标识符。
        id: UUID4,
        /// The initialization timestamp.
        /// 初始化时间戳。
        ts_init: UnixNanos,
    },
    /// An event message with identifiers and timestamps.
    /// 带有标识符和时间戳的事件消息。
    Event {
        /// The unique identifier for this event.
        /// 此事件的唯一标识符。
        id: UUID4,
        /// The initialization timestamp.
        /// 初始化时间戳。
        ts_init: UnixNanos,
        /// The event timestamp.
        /// 事件时间戳。
        ts_event: UnixNanos,
    },
    /// A request message with an identifier and initialization timestamp.
    /// 带有标识符和初始化时间戳的请求消息。
    Request {
        /// The unique identifier for this request.
        /// 此请求的唯一标识符。
        id: UUID4,
        /// The initialization timestamp.
        /// 初始化时间戳。
        ts_init: UnixNanos,
    },
    /// A response message with identifiers, timestamps, and correlation.
    /// 带有标识符、时间戳和关联信息的响应消息。
    Response {
        /// The unique identifier for this response.
        /// 此响应的唯一标识符。
        id: UUID4,
        /// The initialization timestamp.
        /// 初始化时间戳。
        ts_init: UnixNanos,
        /// The correlation identifier linking this response to a request.
        /// 将此响应与请求关联的关联标识符。
        correlation_id: UUID4,
    },
}
