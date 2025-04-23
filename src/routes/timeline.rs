use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::api::timeline::{TimelineQuery, TimelineResponse, TimelineData};

/// 查询攻击时间线
pub async fn query_timeline(
    State(services): State<AppServices>,
    Json(query): Json<TimelineQuery>,
) -> Result<Json<TimelineResponse>, (StatusCode, String)> {
    info!("路由: 查询攻击时间线，情报ID: {}", query.intelligence_id);
    
    // 调用服务层获取时间线数据
    let timeline = services.timeline.get_timeline(query.intelligence_id).await;
    
    // 转换为API响应模型
    let timeline_data = TimelineData::from(timeline);
    
    // 构建响应
    Ok(Json(TimelineResponse {
        code: 200,
        data: timeline_data,
    }))
} 