use std::sync::Arc;

// 移除未使用的导入
// use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    telemetry::{init_tracing, shutdown_tracer},
    db::{
        ClickHouseClient, 
        // 移除未使用的导入
        // ClickHouseUserEventRepository, 
        // ClickHouseAnalysisResultRepository,
        // UserEventRepository,
        // AnalysisResultRepository,
        // InMemoryUserEventRepository,
        // InMemoryAnalysisResultRepository,
    },
    routes::router::create_router,
    services::AppServices,
};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    /// ClickHouse客户端
    pub client: Option<Arc<crate::db::ClickHouseClient>>,
    /// 应用服务
    pub services: AppServices,
}

/// 初始化数据库
async fn init_database() -> Result<AppState, Box<dyn std::error::Error>> {
    let config = crate::config::get_config();
    
    // 尝试创建ClickHouse客户端
    match ClickHouseClient::new(config.db_config).await {
        Ok(clickhouse_client) => {
            // 创建ClickHouse存储库
            let client = Arc::new(clickhouse_client);
            
            // 创建服务层，并传递数据库客户端
            let services = AppServices::new(Some(client.clone()));
            
            Ok(AppState {
                client: Some(client),
                services,
            })
        },
        Err(e) => {
            info!("ClickHouse连接失败: {:?}，使用内存模式", e);
            
            // 创建服务层，使用None表示无数据库连接
            let services = AppServices::new(None);
            
            Ok(AppState {
                client: None,
                services,
            })
        }
    }
}

/// 运行HTTP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化tracing
    init_tracing()?;
    
    // 获取配置中的服务器地址
    let config = crate::config::get_config();
    let server_addr = config.server_addr.to_string();
    
    // 初始化数据库
    info!("正在初始化数据库连接...");
    let state = match init_database().await {
        Ok(state) => {
            info!("数据库初始化成功");
            state
        },
        Err(e) => {
            info!("数据库初始化失败: {}，使用内存模式运行", e);
            AppState {
                client: None,
                services: AppServices::new(None),
            }
        }
    };
    
    // 构建路由
    let app = create_router(state);

    // 日志记录监听地址
    info!("服务器正在监听: {}", server_addr);

    // 创建TCP监听器
    let listener = TcpListener::bind(server_addr).await?;
    
    // 启动HTTP服务器
    let server = axum::serve(listener, app);
    
    // 运行服务器，并在结束时关闭tracer
    let result = server.await;
    
    // 关闭tracer，确保所有spans都被导出
    shutdown_tracer();
    
    result.map_err(Into::into)
} 