# 情报分析API

一个基于Rust开发的情报分析API服务，集成了OpenTelemetry进行分布式追踪和ClickHouse数据库进行高性能数据存储与分析。

## 功能特点

- 使用Axum框架构建的高性能HTTP API服务
- 集成OpenTelemetry进行分布式追踪，支持性能监控和故障排查
- 支持ClickHouse数据库进行高性能数据分析（支持内存模式运行，无需数据库）
- 提供情报分析相关的RESTful API端点

## API接口列表

本服务提供以下API端点：

- `/system/time` (GET) - 获取系统时间
- `/intelligence/list` (POST) - 查询情报列表
- `/intelligence/related-emails` (POST) - 查询与情报关联的邮件
- `/intelligence/timeline` (POST) - 查询攻击时间线
- `/intelligence/statistics` (POST) - 查询统计数据
- `/intelligence/hit-emails-trend` (POST) - 查询命中邮件趋势

## 安装依赖

项目需要以下依赖：
- Rust环境（推荐使用rustup安装）
- 支持OpenTelemetry OTLP协议的追踪后端（如Jaeger，可选）
- ClickHouse数据库服务器（可选，支持内存模式运行）

## 运行方式

### 1. 启动OpenTelemetry收集器（可选）

如果需要分布式追踪功能，可以使用Docker运行Jaeger作为收集器：

```bash
docker run -d -p4317:4317 -p16686:16686 jaegertracing/all-in-one:latest
```

### 2. 配置环境变量

创建一个`.env`文件，包含以下配置：

```
# 服务器配置
SERVER_IP=127.0.0.1
SERVER_PORT=5000

# Jaeger配置
JAEGER_ENDPOINT=http://localhost:4317

# 数据库配置（可选）
DB_URL=http://localhost:8123
DB_USERNAME=你的用户名
DB_PASSWORD=你的密码
DB_NAME=你的数据库名
```

注意：数据库配置是可选的，如果不配置，系统将以内存模式运行。

### 3. 构建并运行服务

```bash
cargo run
```

服务将在配置的地址上启动（默认为 http://127.0.0.1:5000）。

### 4. 测试API

可以使用curl或其他HTTP客户端工具测试API：

```bash
# 获取系统时间
curl http://127.0.0.1:5000/system/time

# 查询情报列表（示例）
curl -X POST http://127.0.0.1:5000/intelligence/list \
  -H "Content-Type: application/json" \
  -d '{"page": 0, "page_size": 10}'
```

### 5. 查看追踪数据（如果配置了Jaeger）

在浏览器中访问Jaeger UI：http://localhost:16686/

## 问题排查

如果遇到以下问题：

1. 服务无法启动
   - 检查端口是否被占用
   - 确保环境变量配置正确

2. 无法连接到Jaeger
   - 确保Jaeger容器正在运行
   - 检查防火墙是否允许4317端口的连接

3. ClickHouse连接错误
   - 确保ClickHouse服务器正在运行
   - 检查防火墙是否允许数据库端口的连接
   - 确认数据库连接参数配置正确

4. 没有看到追踪数据
   - 确认Jaeger容器运行正常
   - 检查`JAEGER_ENDPOINT`环境变量是否正确
   - 查看应用日志中是否有关于OpenTelemetry的错误

## 项目结构

```
src/
├── config.rs         # 配置管理
├── db/               # 数据库访问
├── models/           # 数据模型
├── routes/           # API路由
├── services/         # 业务服务
├── telemetry.rs      # 分布式追踪配置
├── server.rs         # HTTP服务器
├── lib.rs            # 库入口
└── main.rs           # 主程序入口
```

## 技术栈

- [Axum](https://github.com/tokio-rs/axum) - Tokio的Web框架
- [Tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [OpenTelemetry](https://opentelemetry.io/) - 观测平台
- [tracing-opentelemetry](https://github.com/tokio-rs/tracing-opentelemetry) - Tracing与OpenTelemetry的集成
- [ClickHouse](https://clickhouse.com/) - 高性能列式数据库
- [clickhouse-rs](https://github.com/loyd/clickhouse-rs) - ClickHouse Rust客户端
- [dotenv](https://github.com/dotenv-rs/dotenv) - 环境变量管理 