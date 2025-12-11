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

// Under development
// 开发中
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::backend::feather::RotationConfig;

/// Configuration for streaming live or backtest runs to the catalog in feather format.
/// 用于将实时或回测运行流式传输到目录的 feather 格式配置。
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// The path to the data catalog.
    /// 数据目录的路径。
    catalog_path: String,
    /// The `fsspec` filesystem protocol for the catalog.
    /// 目录的 `fsspec` 文件系统协议。
    fst_protocol: String,
    /// The flush interval (milliseconds) for writing chunks.
    /// 写入块的刷新间隔（毫秒）。
    flush_interval_ms: u64,
    /// If any existing feather files should be replaced.
    /// 是否应替换任何现有的 feather 文件。
    replace_existing: bool,
    /// Rotation config
    /// 轮转配置
    rotation_config: RotationConfig,
}

impl StreamingConfig {
    /// Create a new streaming configuration.
    /// 创建新的流式传输配置。
    #[must_use]
    pub const fn new(
        catalog_path: String,
        fst_protocol: String,
        flush_interval_ms: u64,
        replace_existing: bool,
        rotation_config: RotationConfig,
    ) -> Self {
        Self {
            catalog_path,
            fst_protocol,
            flush_interval_ms,
            replace_existing,
            rotation_config,
        }
    }
}

/// Configuration for a data catalog.
/// 数据目录的配置。
pub struct DataCatalogConfig {
    /// The path to the data catalog.
    /// 数据目录的路径。
    path: String,
    /// The fsspec file system protocol for the data catalog.
    /// 数据目录的 fsspec 文件系统协议。
    fs_protocol: String,
}

impl DataCatalogConfig {
    /// Create a new data catalog configuration.
    /// 创建新的数据目录配置。
    #[must_use]
    pub const fn new(path: String, fs_protocol: String) -> Self {
        Self { path, fs_protocol }
    }
}
