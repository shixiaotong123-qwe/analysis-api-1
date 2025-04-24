use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use tracing::{info, instrument};

use crate::services::AppServices;
use crate::models::domain::intelligence::IntelligenceFilter;
use crate::models::api::intelligence::{IntelligenceQueryParams, IntelligenceListResponse, IntelligenceListItem};

/// 处理情报列表查询请求
#[instrument(skip(services))]
pub async fn list_intelligence(
    State(services): State<AppServices>,
    Json(params): Json<IntelligenceQueryParams>,
) -> Result<Json<IntelligenceListResponse>, (StatusCode, String)> {
    info!("路由: 处理情报查询请求");
    
    // 创建领域过滤器
    let filter = IntelligenceFilter {
        start_time: params.start_time,
        end_time: params.end_time,
        sources: params.sources,
        intelligence_type: params.intelligence_type,
        status: params.status,
        filter: params.filter,
        sort_by: params.sort_by,
        sort_order: params.sort_order,
        page_size: params.page_size,
        page: params.page,
    };
    
    // 调用服务层获取情报列表
    let (total, intelligence_list) = services.intelligence.list_intelligence(filter).await;
    
    // 转换为API响应模型
    let response_items = intelligence_list
        .into_iter()
        .map(IntelligenceListItem::from)
        .collect();
    
    // 构建响应
    Ok(Json(IntelligenceListResponse {
        code: 200,
        data: response_items,
        total,
    }))
} 