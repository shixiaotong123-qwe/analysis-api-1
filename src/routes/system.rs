use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::api::system::SystemTimeResponse;
use crate::models::domain::system::SystemTimeData;

/// 获取系统时间
pub async fn get_system_time(
    State(services): State<AppServices>
) -> Result<Json<SystemTimeResponse>, (StatusCode, String)> {
    info!("路由: 获取系统时间");

    // 调用服务层获取系统时间
    let service_time_data = services.system.get_system_time();
    
    // 转换为领域模型
    let domain_time_data = SystemTimeData::from_service(service_time_data);
    
    // 创建API响应
    let response = SystemTimeResponse::success(domain_time_data);

    Ok(Json(response))
} 