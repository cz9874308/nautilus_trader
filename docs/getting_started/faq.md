# 常见问题 (FAQ)

本 FAQ 回答学习 NautilusTrader 的初学者常见问题。如果您在这里找不到答案，请查看[教程](tutorials/index.md)或在 [Discord](https://discord.gg/NautilusTrader) 上提问。

## 安装

### 问：安装失败，提示 "No module named 'nautilus_trader'"

**答**：确保您已正确安装 NautilusTrader：

```bash
# 使用 pip
pip install nautilus_trader

# 或使用 uv（推荐）
uv pip install nautilus_trader
```

**还要检查**:

-   Python 版本是 3.12-3.14

-   您在正确的虚拟环境中

-   安装完成且没有错误

### 问：从源代码构建失败

**答**：确保您拥有所有构建依赖项：

1.  **Rust 工具链** - 通过 [rustup](https://rustup.rs/) 安装

2.  **Clang** - 编译所需

3.  **Python 开发头文件** - 通常是 `python3-dev` 或 `python3-devel`

查看[安装指南](installation.md)了解详细说明。

## 入门

### 问：我从哪里开始？

**答**：遵循这个学习路径：

1.  [安装 NautilusTrader](installation.md)

2.  [新手教程系列](tutorials/index.md) - 从第 1 课开始

3.  [快速开始指南](quickstart.md) - 运行您的第一个回测

### 问：我完全不懂编程。我还能使用 NautilusTrader 吗？

**答**：NautilusTrader 需要基本的 Python 知识。我们建议：

1.  **先学习 Python 基础** - 变量、函数、类

2.  **跟随我们的教程** - 它们专为初学者设计

3.  **从简单策略开始** - 一开始不要尝试复杂策略

**资源**:

-   [Python Tutorial](https://docs.python.org/3/tutorial/)
-   [Learn Python in 10 minutes](https://www.stavros.io/tutorials/python/)

## 数据

### 问：我在哪里获取用于回测的历史数据？

**答**：几个选项：

1.  **交易所 API** - 许多交易所提供历史数据

2.  **数据提供商** - 如 Databento、Tardis 等服务

3.  **CSV 文件** - 加载您自己的数据文件

4.  **测试数据** - 使用 `TestDataProvider` 进行学习

查看[第 6 课：回测实践](tutorials/第06课_回测实践.md)了解示例。

### 问：如何从 CSV 文件加载数据？

**答**：查看[第 6 课](tutorials/第06课_回测实践.md#example-loading-csv-data)中的示例。

### 问：我的数据有缺口。这会导致问题吗？

**答**：缺口可能导致问题。解决方案：

1.  **填补缺口** - 插值缺失数据

2.  **使用 `validate_data_sequence=True`** - 检测缺口

3.  **在策略中处理** - 检查缺失的 K 线

## 策略

### 问：我的策略没有收到数据。为什么？

**答**：检查这些常见问题：

1.  **您订阅了吗？** - 在 `on_start()` 中调用 `self.subscribe_bars()`

2.  **数据是否已加载？** - 验证数据已添加到引擎

3.  **正确的 K 线类型？** - 确保 K 线类型与您的数据匹配

4.  **检查日志** - 查找订阅错误

### 问：如何在策略中访问当前价格？

**答**：使用缓存：

```python
def on_bar(self, bar: Bar):
    # 获取最新报价
    quote = self.cache.quote(self.instrument_id)
    if quote:
        current_price = quote.mid_price()  # 买卖价平均值
        bid = quote.bid  # 最佳买入价格
        ask = quote.ask  # 最佳卖出价格
```

### 问：如何检查我的当前持仓？

**答**：查询缓存：

```python
def on_bar(self, bar: Bar):
    # 获取持仓
    position = self.cache.position(self.instrument_id)

    if position:
        quantity = position.quantity  # 正数 = 多头，负数 = 空头
        pnl = position.unrealized_pnl(USD)  # 当前盈亏
```

## 订单

### 问：我的订单不断被拒绝。为什么？

**答**：常见原因：

1.  **余额不足** - 资金不足

2.  **订单太大** - 超过限制

3.  **风险检查** - 风险引擎拒绝了它

4.  **交易所限制** - 交易所特定限制

**检查拒绝原因：**

```python
def on_order_rejected(self, event: OrderRejected):
    self.log.warning(f"Rejected: {event.reason}")  # 告诉您原因
```

### 问：如何取消订单？

**答**：使用取消方法：

```python
# 取消特定订单
self.cancel_order(order)

# 取消工具的所有订单
self.cancel_orders(self.instrument_id)

# 取消所有订单
self.cancel_all_orders()
```

### 问：市价单和限价单有什么区别？

**答**：

-   **市价单**：立即以最佳可用价格执行

-   **限价单**：只有当价格达到您指定的水平时才执行

```python
# 市价单
order = self.order_factory.market(...)  # 立即执行

# 限价单
order = self.order_factory.limit(price=Price.from_str("50000"), ...)  # 等待价格
```

## 回测

### 问：我的回测结果似乎太好了。这真实吗？

**答**：可能不是！检查：

1.  **过度拟合** - 策略只在这特定数据上有效

2.  **缺少成本** - 忘记考虑费用

3.  **前瞻偏差** - 使用未来数据

4.  **不真实的执行** - 没有滑点或延迟

查看[第 6 课](tutorials/第06课_回测实践.md#common-backtesting-pitfalls)了解详情。

### 问：如何在回测中添加费用？

**答**：费用通常按交易场所配置。查看交易所适配器文档了解费用配置选项。

## 实盘交易

### 问：我可以对回测和实盘使用相同的代码吗？

**答**：**是的！**这是 NautilusTrader 的关键特性之一。您的策略代码对两者都相同。

唯一的区别是**配置**（回测引擎 vs 交易节点）。

### 问：我应该从模拟交易还是真实资金开始？

**答**：**始终从模拟交易开始！**

-   无风险测试您的策略

-   熟悉实盘交易机制

-   只有在数周成功的模拟交易后才使用真实资金

### 问：如何监控我的实盘策略？

**答**：几个选项：

1.  **日志** - 检查日志文件中的事件

2.  **投资组合查询** - 以编程方式检查持仓

3.  **交易所仪表板** - 使用交易所的 Web 界面

4.  **自定义监控** - 构建您自己的仪表板

查看[第 7 课](tutorials/第07课_实盘部署.md#monitoring)了解示例。

## 错误和调试

### 问：我收到 "ModuleNotFoundError"。我该怎么办？

**答**：这意味着 Python 找不到模块。解决方案：

1.  **安装模块** - `pip install module_name`

2.  **检查虚拟环境** - 确保您在正确的环境中

3.  **检查 Python 路径** - 验证模块在 Python 的搜索路径中

### 问：我的策略崩溃了。如何调试它？

**答**：调试步骤：

1.  **检查日志** - 查找错误消息

2.  **添加日志** - 添加 `self.log.info()` 语句

3.  **先在回测中测试** - 更容易调试

4.  **使用 try-except** - 捕获并记录错误

```python
def on_bar(self, bar: Bar):
    try:
        # 您的代码
        pass
    except Exception as e:
        self.log.error(f"Error in on_bar: {e}")
        # 不要让策略崩溃
```

### 问：如何查看策略中发生的情况？

**答**：使用日志：

```python
def on_bar(self, bar: Bar):
    # 记录信息
    self.log.info(f"Bar received: {bar.close}")
    self.log.debug(f"Detailed info: {bar}")  # 更多细节
    self.log.warning(f"Price is high: {bar.close}")  # 警告
    self.log.error(f"Something wrong: {error}")  # 错误
```

在配置中设置日志级别：

```python
logging=LoggingConfig(
    log_level="DEBUG",  # 显示所有消息
)
```

## 性能

### 问：我的回测很慢。如何加速？

**答**：优化技巧：

1.  **减少数据** - 先在较短的时间段测试

2.  **简化策略** - 删除不必要的计算

3.  **使用发布模式** - 以发布模式编译（更快）

4.  **检查数据大小** - 大型数据集需要更长时间

### 问：我可以同时运行多个策略吗？

**答**：是的！将多个策略添加到同一个节点：

```python
# 添加多个策略
node.trader.add_strategy(strategy1)
node.trader.add_strategy(strategy2)
node.trader.add_strategy(strategy3)
```

## 获取帮助

### 问：我在哪里可以获得帮助？

**答**：几个资源：

1.  **文档** - [nautilustrader.io/docs](https://nautilustrader.io/docs/)

2.  **Discord** - [加入社区](https://discord.gg/NautilusTrader)

3.  **GitHub Issues** - [报告错误](https://github.com/nautechsystems/nautilus_trader/issues)

4.  **教程** - [新手教程系列](tutorials/index.md)

### 问：如何报告错误？

**答**：使用 GitHub Issues：

1.  **搜索现有问题** - 检查是否已报告

2.  **创建最小示例** - 使用[最小可复现示例](../../examples/other/minimal_reproducible_example/)

3.  **包含详细信息** - Python 版本、NautilusTrader 版本、错误消息

4.  **提供日志** - 包含相关日志输出

---

## 还有问题？

-   查看[教程](tutorials/index.md)了解分步指南

-   阅读[概念指南](../../concepts/index.md)以获得更深入的理解

-   在 [Discord](https://discord.gg/NautilusTrader) 上提问 - 社区很有帮助！

-   搜索 [GitHub Issues](https://github.com/nautechsystems/nautilus_trader/issues) 查找类似问题
