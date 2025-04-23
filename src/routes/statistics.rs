use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::api::statistics::{
    StatisticsQuery, StatisticsResponse, StatisticsItemResponse
};

/// 查询统计数据
pub async fn query_statistics(
    State(services): State<AppServices>,
    Json(query): Json<StatisticsQuery>,
) -> Result<Json<StatisticsResponse>, (StatusCode, String)> {
    info!(
        "路由: 查询统计数据: start_time={}, end_time={}", 
        query.start_time, 
        query.end_time
    );

    // 调用服务层获取统计数据
    let stats_items = services.statistics.get_statistics(
        query.start_time, 
        query.end_time
    ).await;
    
    // 转换为API响应模型
    let response_items = stats_items
        .into_iter()
        .map(StatisticsItemResponse::from)
        .collect();

    Ok(Json(StatisticsResponse {
        code: 200,
        data: response_items,
    }))
} 