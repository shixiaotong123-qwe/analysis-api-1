use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::domain::email::EmailFilter;
use crate::models::api::email::{RelatedEmailsQuery, RelatedEmailsResponse, EmailResponse};

/// 查询关联邮件
pub async fn query_related_emails(
    State(services): State<AppServices>,
    Json(query): Json<RelatedEmailsQuery>,
) -> Result<Json<RelatedEmailsResponse>, (StatusCode, String)> {
    info!(
        "路由: 查询关联邮件: intelligence_id={}",
        query.intelligence_id
    );

    // 创建领域过滤器
    let filter = EmailFilter {
        start_time: query.start_time,
        end_time: query.end_time,
        intelligence_id: query.intelligence_id,
        status: query.status,
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(10),
    };

    // 调用服务层获取关联邮件
    let (total, emails) = services.email.get_related_emails(filter).await;
    
    // 转换为API响应模型
    let email_responses = emails
        .into_iter()
        .map(EmailResponse::from)
        .collect();

    // 构建响应
    Ok(Json(RelatedEmailsResponse {
        code: 200,
        total,
        data: email_responses,
    }))
} 