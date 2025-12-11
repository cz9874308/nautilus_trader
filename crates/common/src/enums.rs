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

//! Enumerations for common components.
//! 通用组件的枚举。

use log::Level;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, FromRepr};

/// The state of a component within the system.
/// 系统中组件的状态。
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum ComponentState {
    /// When a component is instantiated, but not yet ready to fulfill its specification.
    /// 当组件被实例化但尚未准备好履行其规范时。
    #[default]
    PreInitialized = 0,
    /// When a component is able to be started.
    /// 当组件能够启动时。
    Ready = 1,
    /// When a component is executing its actions on `start`.
    /// 当组件正在执行其 `start` 操作时。
    Starting = 2,
    /// When a component is operating normally and can fulfill its specification.
    /// 当组件正常运行并能够履行其规范时。
    Running = 3,
    /// When a component is executing its actions on `stop`.
    /// 当组件正在执行其 `stop` 操作时。
    Stopping = 4,
    /// When a component has successfully stopped.
    /// 当组件已成功停止时。
    Stopped = 5,
    /// When a component is started again after its initial start.
    /// 当组件在初始启动后再次启动时。
    Resuming = 6,
    /// When a component is executing its actions on `reset`.
    /// 当组件正在执行其 `reset` 操作时。
    Resetting = 7,
    /// When a component is executing its actions on `dispose`.
    /// 当组件正在执行其 `dispose` 操作时。
    Disposing = 8,
    /// When a component has successfully shut down and released all of its resources.
    /// 当组件已成功关闭并释放其所有资源时。
    Disposed = 9,
    /// When a component is executing its actions on `degrade`.
    /// 当组件正在执行其 `degrade` 操作时。
    Degrading = 10,
    /// When a component has successfully degraded and may not meet its full specification.
    /// 当组件已成功降级且可能无法满足其完整规范时。
    Degraded = 11,
    /// When a component is executing its actions on `fault`.
    /// 当组件正在执行其 `fault` 操作时。
    Faulting = 12,
    /// When a component has successfully shut down due to a detected fault.
    /// 当组件由于检测到故障而成功关闭时。
    Faulted = 13,
}

impl ComponentState {
    /// Returns the variant name in a formatted string.
    /// 返回格式化字符串中的变体名称。
    pub fn variant_name(&self) -> String {
        let s = self.to_string();
        format!("{}{}", s[0..1].to_uppercase(), s[1..].to_lowercase())
    }
}

/// A trigger condition for a component within the system.
/// 系统中组件的触发条件。
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum ComponentTrigger {
    /// A trigger for the component to initialize.
    /// 组件初始化的触发器。
    Initialize = 1,
    /// A trigger for the component to start.
    /// 组件启动的触发器。
    Start = 2,
    /// A trigger when the component has successfully started.
    /// 组件成功启动时的触发器。
    StartCompleted = 3,
    /// A trigger for the component to stop.
    /// 组件停止的触发器。
    Stop = 4,
    /// A trigger when the component has successfully stopped.
    /// 组件成功停止时的触发器。
    StopCompleted = 5,
    /// A trigger for the component to resume (after being stopped).
    /// 组件恢复的触发器（在停止后）。
    Resume = 6,
    /// A trigger when the component has successfully resumed.
    /// 组件成功恢复时的触发器。
    ResumeCompleted = 7,
    /// A trigger for the component to reset.
    /// 组件重置的触发器。
    Reset = 8,
    /// A trigger when the component has successfully reset.
    /// 组件成功重置时的触发器。
    ResetCompleted = 9,
    /// A trigger for the component to dispose and release resources.
    /// 组件释放和释放资源的触发器。
    Dispose = 10,
    /// A trigger when the component has successfully disposed.
    /// 组件成功释放时的触发器。
    DisposeCompleted = 11,
    /// A trigger for the component to degrade.
    /// 组件降级的触发器。
    Degrade = 12,
    /// A trigger when the component has successfully degraded.
    DegradeCompleted = 13,
    /// A trigger for the component to fault.
    Fault = 14,
    /// A trigger when the component has successfully faulted.
    FaultCompleted = 15,
}

/// Represents the environment context for a Nautilus system.
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum Environment {
    Backtest,
    Sandbox,
    Live,
}

/// The log level for log messages.
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum LogLevel {
    /// The **OFF** log level. A level lower than all other log levels (off).
    #[strum(serialize = "OFF")]
    #[serde(rename = "OFF")]
    Off = 0,
    /// The **TRACE** log level. Only available in Rust for debug/development builds.
    #[strum(serialize = "TRACE")]
    #[serde(rename = "TRACE")]
    Trace = 1,
    /// The **DEBUG** log level.
    #[strum(serialize = "DEBUG")]
    #[serde(rename = "DEBUG")]
    Debug = 2,
    /// The **INFO** log level.
    #[strum(serialize = "INFO")]
    #[serde(rename = "INFO")]
    Info = 3,
    /// The **WARNING** log level.
    #[strum(serialize = "WARN", serialize = "WARNING")]
    #[serde(rename = "WARNING")]
    Warning = 4,
    /// The **ERROR** log level.
    #[strum(serialize = "ERROR")]
    #[serde(rename = "ERROR")]
    Error = 5,
}

/// The log color for log messages.
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum LogColor {
    /// The default/normal log color.
    #[strum(serialize = "NORMAL")]
    Normal = 0,
    /// The green log color, typically used with [`LogLevel::Info`] log levels and associated with success events.
    #[strum(serialize = "GREEN")]
    Green = 1,
    /// The blue log color, typically used with [`LogLevel::Info`] log levels and associated with user actions.
    #[strum(serialize = "BLUE")]
    Blue = 2,
    /// The magenta log color, typically used with [`LogLevel::Info`] log levels.
    #[strum(serialize = "MAGENTA")]
    Magenta = 3,
    /// The cyan log color, typically used with [`LogLevel::Info`] log levels.
    #[strum(serialize = "CYAN")]
    Cyan = 4,
    /// The yellow log color, typically used with [`LogLevel::Warning`] log levels.
    #[strum(serialize = "YELLOW")]
    Yellow = 5,
    /// The red log color, typically used with [`LogLevel::Error`] level.
    #[strum(serialize = "RED")]
    Red = 6,
}

impl LogColor {
    #[must_use]
    pub const fn as_ansi(&self) -> &str {
        match *self {
            Self::Normal => "",
            Self::Green => "\x1b[92m",
            Self::Blue => "\x1b[94m",
            Self::Magenta => "\x1b[35m",
            Self::Cyan => "\x1b[36m",
            Self::Yellow => "\x1b[1;33m",
            Self::Red => "\x1b[1;31m",
        }
    }
}

impl From<u8> for LogColor {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Magenta,
            4 => Self::Cyan,
            5 => Self::Yellow,
            6 => Self::Red,
            _ => Self::Normal,
        }
    }
}

impl From<Level> for LogColor {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Red,
            Level::Warn => Self::Yellow,
            Level::Info => Self::Normal,
            Level::Debug => Self::Normal,
            Level::Trace => Self::Normal,
        }
    }
}

/// An ANSI log line format specifier.
/// This is used for formatting log messages with ANSI escape codes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromRepr, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum LogFormat {
    /// Header log format. This ANSI escape code is used for magenta text color,
    /// often used for headers or titles in the log output.
    #[strum(serialize = "\x1b[95m")]
    Header,

    /// Endc log format. This ANSI escape code is used to reset all format attributes
    /// to their defaults. It should be used after applying other formats.
    #[strum(serialize = "\x1b[0m")]
    Endc,

    /// Bold log format. This ANSI escape code is used to make the text bold in the log output.
    #[strum(serialize = "\x1b[1m")]
    Bold,

    /// Underline log format. This ANSI escape code is used to underline the text in the log output.
    #[strum(serialize = "\x1b[4m")]
    Underline,
}

/// The serialization encoding.
#[repr(C)]
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromRepr,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(eq, eq_int, module = "nautilus_trader.core.nautilus_pyo3.common.enums")
)]
pub enum SerializationEncoding {
    /// The MessagePack encoding.
    #[serde(rename = "msgpack")]
    MsgPack = 0,
    /// The JavaScript Object Notation (JSON) encoding.
    #[serde(rename = "json")]
    Json = 1,
}
