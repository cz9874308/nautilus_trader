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

use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use ahash::AHashMap;
use nautilus_common::{cache::Cache, clock::Clock};
use nautilus_data::client::DataClient;
use nautilus_execution::client::ExecutionClient;

/// Configuration for creating client instances.
/// 用于创建客户端实例的配置。
///
/// This trait allows different client types to provide their configuration
/// in a type-safe manner while still being usable in generic factory contexts.
/// 此 trait 允许不同的客户端类型以类型安全的方式提供其配置，
/// 同时仍可在通用工厂上下文中使用。
pub trait ClientConfig: Debug {
    /// Return the configuration as a trait object.
    /// 将配置作为 trait 对象返回。
    fn as_any(&self) -> &dyn Any;
}

/// Factory trait for creating data client instances.
/// 用于创建数据客户端实例的工厂 trait。
///
/// Implementations of this trait should create specific data client types
/// (e.g., Binance, Bybit, Databento) based on the provided configuration.
/// 此 trait 的实现应根据提供的配置创建特定的数据客户端类型
/// （例如，Binance、Bybit、Databento）。
pub trait DataClientFactory: Debug {
    /// Create a new data client instance.
    /// 创建新的数据客户端实例。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if client creation fails.
    /// 如果客户端创建失败，则返回错误。
    fn create(
        &self,
        name: &str,
        config: &dyn ClientConfig,
        cache: Rc<RefCell<Cache>>,
        clock: Rc<RefCell<dyn Clock>>,
    ) -> anyhow::Result<Box<dyn DataClient>>;

    /// Returns the name of this factory.
    /// 返回此工厂的名称。
    fn name(&self) -> &str;

    /// Returns the supported configuration type name for this factory.
    /// 返回此工厂支持的配置类型名称。
    fn config_type(&self) -> &str;
}

/// Factory trait for creating execution client instances.
/// 用于创建执行客户端实例的工厂 trait。
///
/// Implementations of this trait should create specific execution client types
/// (e.g., Binance, Bybit, Interactive Brokers) based on the provided configuration.
/// 此 trait 的实现应根据提供的配置创建特定的执行客户端类型
/// （例如，Binance、Bybit、Interactive Brokers）。
pub trait ExecutionClientFactory: Debug {
    /// Create a new execution client instance.
    /// 创建新的执行客户端实例。
    ///
    /// # Errors
    /// # 错误
    ///
    /// Returns an error if client creation fails.
    /// 如果客户端创建失败，则返回错误。
    fn create(
        &self,
        name: &str,
        config: &dyn ClientConfig,
        cache: Rc<RefCell<Cache>>,
        clock: Rc<RefCell<dyn Clock>>,
    ) -> anyhow::Result<Box<dyn ExecutionClient>>;

    /// Returns the name of this factory.
    /// 返回此工厂的名称。
    fn name(&self) -> &str;

    /// Returns the supported configuration type name for this factory.
    /// 返回此工厂支持的配置类型名称。
    fn config_type(&self) -> &str;
}

/// Registry for managing data client factories.
/// 用于管理数据客户端工厂的注册表。
///
/// Allows dynamic registration and lookup of factories by name,
/// enabling a plugin-like architecture for different data providers.
/// 允许按名称动态注册和查找工厂，为不同的数据提供者启用类似插件的架构。
#[derive(Debug, Default)]
pub struct DataClientFactoryRegistry {
    factories: AHashMap<String, Box<dyn DataClientFactory>>,
}

impl DataClientFactoryRegistry {
    /// Creates a new empty registry.
    /// 创建一个新的空注册表。
    #[must_use]
    pub fn new() -> Self {
        Self {
            factories: AHashMap::new(),
        }
    }

    /// Registers a data client factory.
    ///
    /// # Errors
    ///
    /// Returns an error if a factory with the same name is already registered.
    pub fn register(
        &mut self,
        name: String,
        factory: Box<dyn DataClientFactory>,
    ) -> anyhow::Result<()> {
        if self.factories.contains_key(&name) {
            anyhow::bail!("Data client factory '{name}' is already registered");
        }

        self.factories.insert(name, factory);
        Ok(())
    }

    /// Gets a registered factory by name.
    ///
    /// # Returns
    ///
    /// The factory if found, None otherwise.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&dyn DataClientFactory> {
        self.factories.get(name).map(std::convert::AsRef::as_ref)
    }

    /// Gets a list of all registered factory names.
    #[must_use]
    pub fn names(&self) -> Vec<&String> {
        self.factories.keys().collect()
    }

    /// Checks if a factory is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

/// Registry for managing execution client factories.
///
/// Allows dynamic registration and lookup of factories by name,
/// enabling a plugin-like architecture for different execution providers.
#[derive(Debug, Default)]
pub struct ExecutionClientFactoryRegistry {
    factories: AHashMap<String, Box<dyn ExecutionClientFactory>>,
}

impl ExecutionClientFactoryRegistry {
    /// Creates a new empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            factories: AHashMap::new(),
        }
    }

    /// Registers an execution client factory.
    ///
    /// # Errors
    ///
    /// Returns an error if a factory with the same name is already registered.
    pub fn register(
        &mut self,
        name: String,
        factory: Box<dyn ExecutionClientFactory>,
    ) -> anyhow::Result<()> {
        if self.factories.contains_key(&name) {
            anyhow::bail!("Execution client factory '{name}' is already registered");
        }

        self.factories.insert(name, factory);
        Ok(())
    }

    /// Gets a registered factory by name (if found).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&dyn ExecutionClientFactory> {
        self.factories.get(name).map(std::convert::AsRef::as_ref)
    }

    /// Gets a list of all registered factory names.
    #[must_use]
    pub fn names(&self) -> Vec<&String> {
        self.factories.keys().collect()
    }

    /// Checks if a factory is registered.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::any::Any;

    use rstest::*;

    use super::*;

    // Mock configuration for testing
    #[derive(Debug)]
    struct MockConfig {
        #[allow(dead_code)]
        value: String,
    }

    impl ClientConfig for MockConfig {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Mock data client factory for testing
    #[derive(Debug)]
    struct MockDataClientFactory;

    impl DataClientFactory for MockDataClientFactory {
        fn create(
            &self,
            _name: &str,
            _config: &dyn ClientConfig,
            _cache: Rc<RefCell<Cache>>,
            _clock: Rc<RefCell<dyn Clock>>,
        ) -> anyhow::Result<Box<dyn DataClient>> {
            // This would create a real client in practice
            Err(anyhow::anyhow!("Mock factory - not implemented"))
        }

        fn name(&self) -> &'static str {
            "mock"
        }

        fn config_type(&self) -> &'static str {
            "MockConfig"
        }
    }

    #[rstest]
    fn test_data_client_factory_registry() {
        let mut registry = DataClientFactoryRegistry::new();

        // Test empty registry
        assert!(registry.names().is_empty());
        assert!(!registry.contains("mock"));
        assert!(registry.get("mock").is_none());

        // Register factory
        let factory = Box::new(MockDataClientFactory);
        registry.register("mock".to_string(), factory).unwrap();

        // Test after registration
        assert_eq!(registry.names().len(), 1);
        assert!(registry.contains("mock"));
        assert!(registry.get("mock").is_some());

        // Test duplicate registration fails
        let factory2 = Box::new(MockDataClientFactory);
        let result = registry.register("mock".to_string(), factory2);
        assert!(result.is_err());
    }

    #[rstest]
    fn test_empty_data_client_factory_registry() {
        let registry = DataClientFactoryRegistry::new();

        // Test empty registry
        assert!(registry.names().is_empty());
        assert!(!registry.contains("mock"));
        assert!(registry.get("mock").is_none());
    }

    #[rstest]
    fn test_empty_execution_client_factory_registry() {
        let registry = ExecutionClientFactoryRegistry::new();

        // Test empty registry
        assert!(registry.names().is_empty());
        assert!(!registry.contains("mock"));
        assert!(registry.get("mock").is_none());
    }
}
