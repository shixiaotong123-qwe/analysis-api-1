use axum::{
    body::{Body},
    extract::{Json, State},
    http::{ StatusCode, header},
    response::{Response},
};
use tracing::info;

use crate::models::api::email::{EmailResponse, RelatedEmailsQuery, RelatedEmailsResponse, EmailDetailQuery, EmailDetailResponse, EmailDetailData, DownloadFileRequest, FileType};
use crate::models::domain::email::EmailFilter;
use crate::services::AppServices;

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
    let email_responses = emails.into_iter().map(EmailResponse::from).collect();

    // 构建响应
    Ok(Json(RelatedEmailsResponse {
        code: 200,
        total,
        data: email_responses,
    }))
}

/// 下载文件（邮件EML或附件）
pub async fn download_file(
    State(services): State<AppServices>,
    Json(request): Json<DownloadFileRequest>,
) -> Response<Body> {
    // 根据文件类型记录不同的日志
    match request.file_type {
        FileType::Eml => {
            info!("路由: 下载邮件EML: id={}, 加密={}", request.id, request.encrypted);
        },
        FileType::Attachment => {
            info!(
                "路由: 下载附件: id={}, 文件路径={:?}, 加密={}",
                request.id, request.file_path, request.encrypted
            );
        }
    }

    // 确认附件类型时是否提供了文件路径
    if request.file_type == FileType::Attachment && request.file_path.is_none() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("下载附件时必须提供文件路径"))
            .unwrap();
    }

    // 如果请求加密但未提供密码
    if request.encrypted && request.password.is_none() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("加密下载时必须提供密码"))
            .unwrap();
    }

    // 获取文件路径（如果有）
    let file_path = request.file_path.as_deref();
    
    // 对文件路径进行URL解码（如果有）
    let decoded_path = match file_path {
        Some(path) => match urlencoding::decode(path) {
            Ok(decoded) => Some(decoded.into_owned()),
            Err(_) => Some(path.to_string()),
        },
        None => None,
    };

    // 调用服务层下载文件
    match services.email.download_file(
        &request.id,
        request.file_type == FileType::Eml,
        decoded_path.as_deref(),
        request.encrypted,
        request.password.as_deref(),
    ).await {
        Ok((file_data, filename, content_type)) => {
            // 构建成功响应
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(file_data))
                .unwrap()
        }
        Err(e) => {
            // 构建错误响应
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(format!("下载文件失败: {}", e)))
                .unwrap()
        }
    }
}

/// 获取邮件详情
pub async fn get_email_detail(
    State(services): State<AppServices>,
    Json(query): Json<EmailDetailQuery>,
) -> Result<Json<EmailDetailResponse>, (StatusCode, String)> {
    info!("路由: 获取邮件详情: email_id={}", query.email_id);

    // 调用服务层获取邮件详情
    match services.email.get_email_detail(&query.email_id).await {
        Ok(email) => {
            // 转换为API响应模型
            Ok(Json(EmailDetailResponse {
                code: 200,
                data: EmailDetailData::from(email),
            }))
        }
        Err(e) => {
            // 返回错误响应
            Err((StatusCode::NOT_FOUND, format!("邮件未找到: {}", e)))
        }
    }
}
