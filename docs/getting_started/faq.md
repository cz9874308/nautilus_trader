# Frequently Asked Questions (FAQ) / 常见问题 (FAQ)

This FAQ addresses common questions from beginners learning NautilusTrader. If you don't find your answer here, check the [tutorials](tutorials/index.md) or ask on [Discord](https://discord.gg/NautilusTrader).

本 FAQ 回答学习 NautilusTrader 的初学者常见问题。如果您在这里找不到答案，请查看[教程](tutorials/index.md)或在 [Discord](https://discord.gg/NautilusTrader) 上提问。

## Installation / 安装

### Q: Installation fails with "No module named 'nautilus_trader'" / 问：安装失败，提示 "No module named 'nautilus_trader'"

**A**: Make sure you've installed NautilusTrader correctly:

**答**：确保您已正确安装 NautilusTrader：

```bash
# Using pip / 使用 pip
pip install nautilus_trader

# Or using uv (recommended) / 或使用 uv（推荐）
uv pip install nautilus_trader
```

**Also check / 还要检查**:

-   Python version is 3.12-3.14
    Python 版本是 3.12-3.14

-   You're in the correct virtual environment
    您在正确的虚拟环境中

-   Installation completed without errors
    安装完成且没有错误

### Q: Build from source fails / 问：从源代码构建失败

**A**: Ensure you have all build dependencies:

**答**：确保您拥有所有构建依赖项：

1.  **Rust toolchain** - Install via [rustup](https://rustup.rs/)
    **Rust 工具链** - 通过 [rustup](https://rustup.rs/) 安装

2.  **Clang** - Required for compilation
    **Clang** - 编译所需

3.  **Python development headers** - Usually `python3-dev` or `python3-devel`
    **Python 开发头文件** - 通常是 `python3-dev` 或 `python3-devel`

See the [Installation Guide](installation.md) for detailed instructions.

查看[安装指南](installation.md)了解详细说明。

## Getting Started / 入门

### Q: Where do I start? / 问：我从哪里开始？

**A**: Follow this learning path:

**答**：遵循这个学习路径：

1.  [Install NautilusTrader](installation.md)
    [安装 NautilusTrader](installation.md)

2.  [Beginner Tutorial Series](tutorials/index.md) - Start with Lesson 1
    [新手教程系列](tutorials/index.md) - 从第 1 课开始

3.  [Quickstart Guide](quickstart.md) - Run your first backtest
    [快速开始指南](quickstart.md) - 运行您的第一个回测

### Q: I'm completely new to programming. Can I still use NautilusTrader? / 问：我完全不懂编程。我还能使用 NautilusTrader 吗？

**A**: NautilusTrader requires basic Python knowledge. We recommend:

**答**：NautilusTrader 需要基本的 Python 知识。我们建议：

1.  **Learn Python basics first** - Variables, functions, classes
    **先学习 Python 基础** - 变量、函数、类

2.  **Follow our tutorials** - They're designed for beginners
    跟随我们的教程 - 它们专为初学者设计

3.  **Start with simple strategies** - Don't try complex strategies at first
    从简单策略开始 - 一开始不要尝试复杂策略

**Resources / 资源**:

-   [Python Tutorial](https://docs.python.org/3/tutorial/)
-   [Learn Python in 10 minutes](https://www.stavros.io/tutorials/python/)

## Data / 数据

### Q: Where do I get historical data for backtesting? / 问：我在哪里获取用于回测的历史数据？

**A**: Several options:

**答**：几个选项：

1.  **Exchange APIs** - Many exchanges provide historical data
    **交易所 API** - 许多交易所提供历史数据

2.  **Data Providers** - Services like Databento, Tardis
    **数据提供商** - 如 Databento、Tardis 等服务

3.  **CSV Files** - Load your own data files
    **CSV 文件** - 加载您自己的数据文件

4.  **Test Data** - Use `TestDataProvider` for learning
    **测试数据** - 使用 `TestDataProvider` 进行学习

See [Lesson 6: Backtesting Practice](tutorials/第06课_回测实践.md) for examples.

查看[第 6 课：回测实践](tutorials/第06课_回测实践.md)了解示例。

### Q: How do I load data from a CSV file? / 问：如何从 CSV 文件加载数据？

**A**: See the example in [Lesson 6](tutorials/第06课_回测实践.md#example-loading-csv-data).

**答**：查看[第 6 课](tutorials/第06课_回测实践.md#example-loading-csv-data)中的示例。

### Q: My data has gaps. Will this cause problems? / 问：我的数据有缺口。这会导致问题吗？

**A**: Gaps can cause issues. Solutions:

**答**：缺口可能导致问题。解决方案：

1.  **Fill gaps** - Interpolate missing data
    **填补缺口** - 插值缺失数据

2.  **Use `validate_data_sequence=True`** - Detects gaps
    使用 `validate_data_sequence=True` - 检测缺口

3.  **Handle in strategy** - Check for missing bars
    在策略中处理 - 检查缺失的 K 线

## Strategies / 策略

### Q: My strategy isn't receiving data. Why? / 问：我的策略没有收到数据。为什么？

**A**: Check these common issues:

**答**：检查这些常见问题：

1.  **Did you subscribe?** - Call `self.subscribe_bars()` in `on_start()`
    **您订阅了吗？** - 在 `on_start()` 中调用 `self.subscribe_bars()`

2.  **Is data loaded?** - Verify data was added to the engine
    数据是否已加载？ - 验证数据已添加到引擎

3.  **Correct bar type?** - Ensure bar type matches your data
    正确的 K 线类型？ - 确保 K 线类型与您的数据匹配

4.  **Check logs** - Look for subscription errors
    检查日志 - 查找订阅错误

### Q: How do I access current price in my strategy? / 问：如何在策略中访问当前价格？

**A**: Use the cache:

**答**：使用缓存：

```python
def on_bar(self, bar: Bar):
    # Get latest quote / 获取最新报价
    quote = self.cache.quote(self.instrument_id)
    if quote:
        current_price = quote.mid_price()  # Average of bid/ask / 买卖价平均值
        bid = quote.bid  # Best buy price / 最佳买入价格
        ask = quote.ask  # Best sell price / 最佳卖出价格
```

### Q: How do I check my current position? / 问：如何检查我的当前持仓？

**A**: Query the cache:

**答**：查询缓存：

```python
def on_bar(self, bar: Bar):
    # Get position / 获取持仓
    position = self.cache.position(self.instrument_id)

    if position:
        quantity = position.quantity  # Positive = long, Negative = short
        pnl = position.unrealized_pnl(USD)  # Current profit/loss
```

## Orders / 订单

### Q: My orders keep getting rejected. Why? / 问：我的订单不断被拒绝。为什么？

**A**: Common reasons:

**答**：常见原因：

1.  **Insufficient balance** - Not enough money
    **余额不足** - 资金不足

2.  **Order too large** - Exceeds limits
    **订单太大** - 超过限制

3.  **Risk checks** - Risk engine rejected it
    **风险检查** - 风险引擎拒绝了它

4.  **Exchange limits** - Exchange-specific restrictions
    **交易所限制** - 交易所特定限制

**Check the rejection reason:**
**检查拒绝原因：**

```python
def on_order_rejected(self, event: OrderRejected):
    self.log.warning(f"Rejected: {event.reason}")  # Tells you why / 告诉您原因
```

### Q: How do I cancel an order? / 问：如何取消订单？

**A**: Use the cancel method:

**答**：使用取消方法：

```python
# Cancel specific order / 取消特定订单
self.cancel_order(order)

# Cancel all orders for instrument / 取消工具的所有订单
self.cancel_orders(self.instrument_id)

# Cancel all orders / 取消所有订单
self.cancel_all_orders()
```

### Q: What's the difference between market and limit orders? / 问：市价单和限价单有什么区别？

**A**:

**答**：

-   **Market Order / 市价单**: Executes immediately at best available price
    **市价单**：立即以最佳可用价格执行

-   **Limit Order / 限价单**: Only executes if price reaches your specified level
    **限价单**：只有当价格达到您指定的水平时才执行

```python
# Market order / 市价单
order = self.order_factory.market(...)  # Executes now / 立即执行

# Limit order / 限价单
order = self.order_factory.limit(price=Price.from_str("50000"), ...)  # Waits for price / 等待价格
```

## Backtesting / 回测

### Q: My backtest results seem too good. Is this realistic? / 问：我的回测结果似乎太好了。这真实吗？

**A**: Probably not! Check for:

**答**：可能不是！检查：

1.  **Overfitting** - Strategy only works on this specific data
    **过度拟合** - 策略只在这特定数据上有效

2.  **Missing costs** - Forgot to account for fees
    **缺少成本** - 忘记考虑费用

3.  **Look-ahead bias** - Using future data
    **前瞻偏差** - 使用未来数据

4.  **Unrealistic execution** - No slippage or delays
    **不真实的执行** - 没有滑点或延迟

See [Lesson 6](tutorials/第06课_回测实践.md#common-backtesting-pitfalls) for details.

查看[第 6 课](tutorials/第06课_回测实践.md#common-backtesting-pitfalls)了解详情。

### Q: How do I add fees to my backtest? / 问：如何在回测中添加费用？

**A**: Fees are typically configured per venue. Check the exchange adapter documentation for fee configuration options.

**答**：费用通常按交易场所配置。查看交易所适配器文档了解费用配置选项。

## Live Trading / 实盘交易

### Q: Can I use the same code for backtest and live? / 问：我可以对回测和实盘使用相同的代码吗？

**A**: **Yes!** This is one of NautilusTrader's key features. Your strategy code is identical for both.

**答**：**是的！**这是 NautilusTrader 的关键特性之一。您的策略代码对两者都相同。

The only difference is the **configuration** (backtest engine vs trading node).

唯一的区别是**配置**（回测引擎 vs 交易节点）。

### Q: Should I start with paper trading or real money? / 问：我应该从模拟交易还是真实资金开始？

**A**: **Always start with paper trading!**

**答**：**始终从模拟交易开始！**

-   Test your strategy without risk
    无风险测试您的策略

-   Get familiar with live trading mechanics
    熟悉实盘交易机制

-   Only use real money after weeks of successful paper trading
    只有在数周成功的模拟交易后才使用真实资金

### Q: How do I monitor my live strategy? / 问：如何监控我的实盘策略？

**A**: Several options:

**答**：几个选项：

1.  **Logs** - Check log files for events
    **日志** - 检查日志文件中的事件

2.  **Portfolio queries** - Programmatically check positions
    **投资组合查询** - 以编程方式检查持仓

3.  **Exchange dashboard** - Use exchange's web interface
    **交易所仪表板** - 使用交易所的 Web 界面

4.  **Custom monitoring** - Build your own dashboard
    **自定义监控** - 构建您自己的仪表板

See [Lesson 7](tutorials/第07课_实盘部署.md#monitoring) for examples.

查看[第 7 课](tutorials/第07课_实盘部署.md#monitoring)了解示例。

## Errors and Debugging / 错误和调试

### Q: I get "ModuleNotFoundError". What do I do? / 问：我收到 "ModuleNotFoundError"。我该怎么办？

**A**: This means Python can't find a module. Solutions:

**答**：这意味着 Python 找不到模块。解决方案：

1.  **Install the module** - `pip install module_name`
    **安装模块** - `pip install module_name`

2.  **Check virtual environment** - Make sure you're in the right environment
    检查虚拟环境 - 确保您在正确的环境中

3.  **Check Python path** - Verify module is in Python's search path
    检查 Python 路径 - 验证模块在 Python 的搜索路径中

### Q: My strategy crashes. How do I debug it? / 问：我的策略崩溃了。如何调试它？

**A**: Debugging steps:

**答**：调试步骤：

1.  **Check logs** - Look for error messages
    检查日志 - 查找错误消息

2.  **Add logging** - Add `self.log.info()` statements
    添加日志 - 添加 `self.log.info()` 语句

3.  **Test in backtest first** - Easier to debug
    先在回测中测试 - 更容易调试

4.  **Use try-except** - Catch and log errors
    使用 try-except - 捕获并记录错误

```python
def on_bar(self, bar: Bar):
    try:
        # Your code / 您的代码
        pass
    except Exception as e:
        self.log.error(f"Error in on_bar: {e}")
        # Don't crash the strategy / 不要让策略崩溃
```

### Q: How do I see what's happening in my strategy? / 问：如何查看策略中发生的情况？

**A**: Use logging:

**答**：使用日志：

```python
def on_bar(self, bar: Bar):
    # Log information / 记录信息
    self.log.info(f"Bar received: {bar.close}")
    self.log.debug(f"Detailed info: {bar}")  # More detail / 更多细节
    self.log.warning(f"Price is high: {bar.close}")  # Warnings / 警告
    self.log.error(f"Something wrong: {error}")  # Errors / 错误
```

Set log level in configuration:

在配置中设置日志级别：

```python
logging=LoggingConfig(
    log_level="DEBUG",  # Shows all messages / 显示所有消息
)
```

## Performance / 性能

### Q: My backtest is slow. How can I speed it up? / 问：我的回测很慢。如何加速？

**A**: Optimization tips:

**答**：优化技巧：

1.  **Reduce data** - Test on shorter time periods first
    减少数据 - 先在较短的时间段测试

2.  **Simplify strategy** - Remove unnecessary calculations
    简化策略 - 删除不必要的计算

3.  **Use release mode** - Compile in release mode (faster)
    使用发布模式 - 以发布模式编译（更快）

4.  **Check data size** - Large datasets take longer
    检查数据大小 - 大型数据集需要更长时间

### Q: Can I run multiple strategies at once? / 问：我可以同时运行多个策略吗？

**A**: Yes! Add multiple strategies to the same node:

**答**：是的！将多个策略添加到同一个节点：

```python
# Add multiple strategies / 添加多个策略
node.trader.add_strategy(strategy1)
node.trader.add_strategy(strategy2)
node.trader.add_strategy(strategy3)
```

## Getting Help / 获取帮助

### Q: Where can I get help? / 问：我在哪里可以获得帮助？

**A**: Several resources:

**答**：几个资源：

1.  **Documentation** - [nautilustrader.io/docs](https://nautilustrader.io/docs/)
    **文档** - [nautilustrader.io/docs](https://nautilustrader.io/docs/)

2.  **Discord** - [Join the community](https://discord.gg/NautilusTrader)
    **Discord** - [加入社区](https://discord.gg/NautilusTrader)

3.  **GitHub Issues** - [Report bugs](https://github.com/nautechsystems/nautilus_trader/issues)
    **GitHub Issues** - [报告错误](https://github.com/nautechsystems/nautilus_trader/issues)

4.  **Tutorials** - [Beginner Tutorial Series](tutorials/index.md)
    **教程** - [新手教程系列](tutorials/index.md)

### Q: How do I report a bug? / 问：如何报告错误？

**A**: Use GitHub Issues:

**答**：使用 GitHub Issues：

1.  **Search existing issues** - Check if it's already reported
    搜索现有问题 - 检查是否已报告

2.  **Create minimal example** - Use the [minimal reproducible example](../../examples/other/minimal_reproducible_example/)
    创建最小示例 - 使用[最小可复现示例](../../examples/other/minimal_reproducible_example/)

3.  **Include details** - Python version, NautilusTrader version, error messages
    包含详细信息 - Python 版本、NautilusTrader 版本、错误消息

4.  **Provide logs** - Include relevant log output
    提供日志 - 包含相关日志输出

---

## Still Have Questions? / 还有问题？

-   Check the [tutorials](tutorials/index.md) for step-by-step guides
    查看[教程](tutorials/index.md)了解分步指南

-   Read the [concepts guides](../../concepts/index.md) for deeper understanding
    阅读[概念指南](../../concepts/index.md)以获得更深入的理解

-   Ask on [Discord](https://discord.gg/NautilusTrader) - The community is helpful!
    在 [Discord](https://discord.gg/NautilusTrader) 上提问 - 社区很有帮助！

-   Search [GitHub Issues](https://github.com/nautechsystems/nautilus_trader/issues) for similar problems
    搜索 [GitHub Issues](https://github.com/nautechsystems/nautilus_trader/issues) 查找类似问题
