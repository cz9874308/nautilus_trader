# 快速开始

要开始使用 NautilusTrader，您需要：

-   已安装 `nautilus_trader` 包的 Python 3.12–3.14 环境。
-   用于回测和/或实盘交易的运行 Python 脚本或 Jupyter notebook 的方式。

## [新手教程系列](tutorials/index.md)

刚接触算法交易或 NautilusTrader？从这里开始！

**新手教程系列**为完全初学者提供分步指南，包含人性化的解释和实际示例。

👉 **[开始学习 →](tutorials/index.md)**

## [安装](installation.md)

**安装**指南将帮助确保 NautilusTrader 在您的机器上正确安装。

## [快速开始](quickstart.md)

**快速开始**提供了设置第一个回测的分步指南。

## 仓库中的示例

[在线文档](https://nautilustrader.io/docs/latest/) 仅显示示例的一个子集。完整集合，请查看 GitHub 上的此仓库。

以下表格按推荐的学习进度列出了示例位置：

| 目录                                                                                 | 内容                                       |
| :----------------------------------------------------------------------------------- | :----------------------------------------- |
| [examples/](https://github.com/nautechsystems/nautilus_trader/tree/develop/examples) | 完全可运行的、自包含的 Python 示例。       |
| [docs/tutorials/](../tutorials/)                                                     | 演示常见工作流的 Jupyter notebook 教程。   |
| [docs/concepts/](../concepts/)                                                       | 概念指南，包含说明关键功能的简洁代码片段。 |
| [nautilus_trader/examples/](../../nautilus_trader/examples/)                         | 基本策略、指标和执行算法的纯 Python 示例。 |
| [tests/unit_tests/](../../tests/unit_tests/)                                         | 涵盖核心功能和边缘情况的单元测试。         |

## 回测 API 级别

NautilusTrader 为回测提供两个不同的 API 级别：

| API 级别 | 描述                                 | 特点                                                                   |
| :------- | :----------------------------------- | :--------------------------------------------------------------------- |
| 高级 API | 使用 `BacktestNode` 和 `TradingNode` | 推荐用于生产：更容易过渡到实盘交易；需要基于 Parquet 的数据目录。      |
| 低级 API | 使用 `BacktestEngine`                | 用于库开发：没有实盘交易路径；直接组件访问；可能鼓励非实盘兼容的模式。 |

:::warning **每个进程一个节点**
由于全局单例状态，不支持在同一进程中并发运行多个 `BacktestNode` 或 `TradingNode` 实例。
支持在运行之间进行适当的清理后顺序执行。

有关详细信息，请参阅[进程和线程](../concepts/architecture.md#processes-and-threads)。
:::

回测涉及在历史数据上运行模拟交易系统。

要开始使用 NautilusTrader 进行回测，您需要首先了解提供的两个不同的 API 级别，以及哪个可能更适合您的预期用例。

:::info
有关选择哪个 API 级别的更多信息，请参阅[回测](../concepts/backtesting.md)指南。
:::

### [回测（低级 API）](backtest_low_level.md)

本教程介绍如何使用数据加载器和数据整理器加载原始数据（Nautilus 外部），然后使用 `BacktestEngine` 使用此数据运行单个回测。

### [回测（高级 API）](backtest_high_level.md)

本教程介绍如何将原始数据（Nautilus 外部）加载到数据目录中，然后使用 `BacktestNode` 使用此数据运行单个回测。

## 在 Docker 中运行

或者，您可以下载一个自包含的 Docker 化 Jupyter notebook 服务器，它不需要任何设置或安装。这是最快的方式来启动并尝试 NautilusTrader。请注意，删除容器也会删除所有数据。

-   要开始，请安装 docker：
    -   转到 [Docker 安装指南](https://docs.docker.com/get-docker/) 并按照说明操作。
-   从终端下载最新镜像：
    -   `docker pull ghcr.io/nautechsystems/jupyterlab:nightly --platform linux/amd64`
-   运行 docker 容器，暴露 jupyter 端口：
    -   `docker run -p 8888:8888 ghcr.io/nautechsystems/jupyterlab:nightly`
-   在 Web 浏览器中打开 `localhost:{port}`：
    -   <http://localhost:8888>

:::info
NautilusTrader 目前超过了 Jupyter notebook 日志记录（stdout 输出）的速率限制，
因此我们在示例中将 `log_level` 设置为 `ERROR`。降低此级别以查看
更多日志记录将导致 notebook 在单元格执行期间挂起。我们目前正在
调查一个修复方案，涉及提高 Jupyter 的配置速率限制，
或限制来自 Nautilus 的日志刷新。

-   <https://github.com/jupyterlab/jupyterlab/issues/12845>
-   <https://github.com/deshaw/jupyterlab-limit-output>

:::
