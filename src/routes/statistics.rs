use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::api::statistics::{
    StatisticsQuery, StatisticsResponse, StatisticsResponseData
};
use crate::models::domain::statistics::StatisticsFilter;

/// 查询统计数据
pub async fn query_statistics(
    State(services): State<AppServices>,
    Json(query): Json<StatisticsQuery>,
) -> Result<Json<StatisticsResponse>, (StatusCode, String)> {
    info!(
        "路由: 查询统计数据: start_time={}, end_time={}, module={}", 
        query.start_time, 
        query.end_time,
        query.module
    );

    // 创建领域过滤器
    let filter = StatisticsFilter {
        start_time: query.start_time,
        end_time: query.end_time,
        module: query.module,
        extras: query.extras,
    };

    // 调用服务层获取统计数据
    let stats_result = services.statistics.get_statistics(filter).await;
    
    // 转换为API响应模型
    let response_data = StatisticsResponseData::from(stats_result);

    Ok(Json(StatisticsResponse {
        code: 200,
        data: response_data,
    }))
} 