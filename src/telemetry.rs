use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace, Resource};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 初始化Tracing
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // 首先尝试HTTP方式初始化OpenTelemetry
    if let Err(e) = init_tracer() {
        info!("HTTP导出器初始化失败: {:?}，尝试使用gRPC", e);
        init_tracer_grpc()?;
    }

    // 设置tracing订阅者
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info")
                .add_directive("analysis_api=debug".parse().unwrap())
                .add_directive("clickhouse=debug".parse().unwrap())
        });

    // 创建格式化层
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // 设置订阅者
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(tracing_opentelemetry::layer())
        .init();

    info!("初始化tracing完成");
    Ok(())
}

/// 初始化OpenTelemetry tracer (HTTP)
fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {
    // 创建OTLP导出器 - 使用reqwest客户端
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("http://localhost:4317")
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary) // 明确指定协议
        .build_span_exporter()?;
    
    // 配置资源信息
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "analysis-api"),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
    ]);
    
    // 配置并初始化全局tracer提供者
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_config(
            opentelemetry_sdk::trace::config()
                .with_sampler(trace::Sampler::AlwaysOn)
                .with_resource(resource)
        )
        .with_simple_exporter(exporter)
        .build();
    
    global::set_tracer_provider(provider);
    
    Ok(())
}

/// 初始化OpenTelemetry tracer (gRPC)
fn init_tracer_grpc() -> Result<(), Box<dyn std::error::Error>> {
    // 创建OTLP导出器 - 使用gRPC
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317")
        .build_span_exporter()?;
    
    // 配置资源信息
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "analysis-api"),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
    ]);
    
    // 配置并初始化全局tracer提供者
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_config(
            opentelemetry_sdk::trace::config()
                .with_sampler(trace::Sampler::AlwaysOn)
                .with_resource(resource)
        )
        .with_simple_exporter(exporter)
        .build();
    
    global::set_tracer_provider(provider);
    
    Ok(())
}

/// 关闭OpenTelemetry tracer
pub fn shutdown_tracer() {
    global::shutdown_tracer_provider();
} 