use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use tracing::info;

use crate::services::AppServices;
use crate::models::{
    WhitelistRequest, BlacklistRequest, ReportRequest, 
    ActionResponse, ActionResponseData,
    WhitelistAction, BlacklistAction, ReportAction
};

/// 处理加白请求
pub async fn add_to_whitelist(
    State(services): State<AppServices>,
    Json(request): Json<WhitelistRequest>,
) -> Result<Json<ActionResponse>, (StatusCode, String)> {
    info!("路由: 处理情报加白请求，情报ID: {}, 情报值: {}", request.intelligence_id, request.value);
    
    // 将API请求转换为领域模型
    let action = WhitelistAction::from(request);
    
    // 调用情报操作服务执行加白操作
    match services.intelligence_action.add_to_whitelist(
        action.intelligence_id,
        action.value,
        action.release_quarantine
    ).await {
        Ok(_) => {
            // 构建成功响应
            Ok(Json(ActionResponse {
                code: 200,
                data: ActionResponseData {
                    msg: "操作成功".to_string(),
                },
            }))
        },
        Err(e) => {
            // 构建错误响应
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("加白操作失败: {}", e)))
        }
    }
}

/// 处理加黑请求
pub async fn add_to_blacklist(
    State(services): State<AppServices>,
    Json(request): Json<BlacklistRequest>,
) -> Result<Json<ActionResponse>, (StatusCode, String)> {
    info!("路由: 处理情报加黑请求，情报ID: {}, 情报值: {}", request.intelligence_id, request.value);
    
    // 将API请求转换为领域模型
    let action = BlacklistAction::from(request);
    
    // 调用情报操作服务执行加黑操作
    match services.intelligence_action.add_to_blacklist(
        action.intelligence_id,
        action.value,
        false // 不再使用quarantine_emails参数
    ).await {
        Ok(_) => {
            // 构建成功响应
            Ok(Json(ActionResponse {
                code: 200,
                data: ActionResponseData {
                    msg: "操作成功".to_string(),
                },
            }))
        },
        Err(e) => {
            // 构建错误响应
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("加黑操作失败: {}", e)))
        }
    }
}

/// 处理情报上报请求
pub async fn report_intelligence(
    State(services): State<AppServices>,
    Json(request): Json<ReportRequest>,
) -> Result<Json<ActionResponse>, (StatusCode, String)> {
    info!("路由: 处理情报上报请求，情报日志ID: {}, 情报ID: {}, 原因: {}", 
        request.id, request.intelligence_id, request.reason);
    
    // 将API请求转换为领域模型
    let action = ReportAction::from(request);
    
    // 调用情报操作服务执行上报操作
    match services.intelligence_action.report_false_positive(
        action.id,
        action.intelligence_id,
        action.reason,
        action.provide_sample_email
    ).await {
        Ok(_) => {
            // 构建成功响应
            Ok(Json(ActionResponse {
                code: 200,
                data: ActionResponseData {
                    msg: "上报成功".to_string(),
                },
            }))
        },
        Err(e) => {
            // 构建错误响应
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("上报操作失败: {}", e)))
        }
    }
} 