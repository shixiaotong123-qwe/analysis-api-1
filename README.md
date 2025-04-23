# analysis-api

一个使用Axum框架构建的HTTP API服务器，集成了OpenTelemetry进行分布式追踪和ClickHouse数据库进行数据存储与分析。

## 功能特点

- 使用Axum构建的HTTP服务器
- 集成OpenTelemetry进行分布式追踪
- 使用ClickHouse数据库进行高性能数据存储和分析
- 提供基本的RESTful API端点

## 安装依赖

项目需要以下依赖：
- Rust环境
- 支持OpenTelemetry OTLP协议的后端收集器（如Jaeger）
- ClickHouse数据库服务器

## 运行方式

### 1. 启动OpenTelemetry收集器

使用Docker运行Jaeger作为收集器：

```bash
docker run -d -p4317:4317 -p16686:16686 jaegertracing/all-in-one:latest
```

### 2. 配置环境变量

创建一个`.env`文件，包含以下配置：

```
# 服务器配置
SERVER_IP=127.0.0.1
SERVER_PORT=6000

# Jaeger配置
JAEGER_ENDPOINT=http://localhost:4317

# 数据库配置
DB_URL=http://localhost:8123
DB_USERNAME=你的用户名
DB_PASSWORD=你的密码
DB_NAME=你的数据库名
```

### 3. 构建并运行服务

```bash
cargo run
```

服务将监听在 http://127.0.0.1:6000

### 4. 测试API

- 访问首页: `GET http://127.0.0.1:6000/`
- Hello World: `GET http://127.0.0.1:6000/hello`
- 创建用户: `POST http://127.0.0.1:6000/users`
- 记录事件: `POST http://127.0.0.1:6000/events`
- 获取事件计数: `POST http://127.0.0.1:6000/events/count`

示例请求创建用户：

```bash
curl -X POST http://127.0.0.1:6000/users \
  -H "Content-Type: application/json" \
  -d '{"username": "测试用户", "email": "test@example.com"}'
```

示例请求记录事件：

```bash
curl -X POST http://127.0.0.1:6000/events \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 1,
    "event_type": "login",
    "payload": {"device": "mobile", "location": "beijing"},
    "ip_address": "192.168.1.1",
    "user_agent": "Mozilla/5.0"
  }'
```

### 5. 查看追踪数据

在浏览器中访问Jaeger UI：http://localhost:16686/

### 6. 查询ClickHouse数据

使用ClickHouse客户端或HTTP接口查询数据：

```bash
# 使用HTTP接口查询示例
curl "http://localhost:8123/?query=SELECT+count(*)+FROM+user_events"
```

## 问题排查

如果遇到以下问题：

1. OpenTelemetry特性相关错误
   - 确保在Cargo.toml中正确配置了features
   - `rt-tokio`特性应该在`opentelemetry_sdk`而不是`opentelemetry`包中

2. 无法连接到Jaeger
   - 确保Jaeger容器正在运行
   - 检查防火墙是否允许4317端口的连接

3. Axum服务器启动错误
   - Axum 0.7+ 版本中，使用`axum::serve(listener, app)`而不是`Server::bind(&addr).serve(app.into_make_service())`
   - 需要先创建一个`TcpListener`再传递给`serve`函数

4. OpenTelemetry导出器配置错误
   - 使用`.build_span_exporter()?`完成导出器的构建
   - 使用`with_config`和`trace::config()`配置采样器和资源

5. HTTP客户端错误 (`Error: ExportFailed(NoHttpClient)`)
   - 在Cargo.toml中添加`reqwest-client`特性：
     ```toml
     opentelemetry-otlp = { version = "0.14", features = ["trace", "http-proto", "reqwest-client"] }
     ```
   - 在代码中明确指定HTTP协议：
     ```rust
     .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
     ```
   - 如果HTTP方法仍然失败，尝试使用gRPC：
     ```toml
     opentelemetry-otlp = { version = "0.14", features = ["trace", "grpc-tonic"] }
     ```
     ```rust
     let exporter = opentelemetry_otlp::new_exporter()
         .tonic()
         .with_endpoint("http://localhost:4317")
         .build_span_exporter()?;
     ```

6. ClickHouse连接错误
   - 确保ClickHouse服务器正在运行
   - 检查防火墙是否允许数据库端口的连接
   - 确保数据库连接参数配置正确

## 数据库架构

### 用户事件表 (user_events)

| 列名 | 类型 | 描述 |
|------|------|------|
| event_id | UUID | 事件唯一标识符 |
| user_id | UInt64 | 用户ID |
| event_type | String | 事件类型 |
| payload | String | 事件数据（JSON格式） |
| timestamp | DateTime | 事件时间 |
| ip_address | Nullable(String) | IP地址 |
| user_agent | Nullable(String) | 用户代理 |

### 分析结果表 (analysis_results)

| 列名 | 类型 | 描述 |
|------|------|------|
| result_id | UUID | 结果唯一标识符 |
| analysis_name | String | 分析名称 |
| result_data | String | 结果数据（JSON格式） |
| created_at | DateTime | 创建时间 |
| updated_at | DateTime | 更新时间 |
| parameters | Nullable(String) | 分析参数（JSON格式） |

## 技术栈

- [Axum](https://github.com/tokio-rs/axum) - Tokio的Web框架
- [Tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [OpenTelemetry](https://opentelemetry.io/) - 观测平台
- [tracing-opentelemetry](https://github.com/tokio-rs/tracing-opentelemetry) - Tracing与OpenTelemetry的集成
- [ClickHouse](https://clickhouse.com/) - 高性能列式数据库
- [clickhouse-rs](https://github.com/loyd/clickhouse-rs) - ClickHouse Rust客户端
- [dotenv](https://github.com/dotenv-rs/dotenv) - 环境变量管理 