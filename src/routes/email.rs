use axum::{
    body::{Body},
    extract::{Json, State},
    http::{ StatusCode, header},
    response::{Response},
};
use tracing::info;

use crate::models::api::email::{EmailResponse, RelatedEmailsQuery, RelatedEmailsResponse};
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

/// POST请求数据结构体 - 下载邮件EML
#[derive(serde::Deserialize)]
pub struct DownloadEmailRequest {
    /// 邮件ID
    pub email_id: String,
}

/// 下载邮件EML
pub async fn download_email_eml(
    State(services): State<AppServices>,
    Json(request): Json<DownloadEmailRequest>,
) -> Response<Body> {
    info!("路由: 下载邮件EML: email_id={}", request.email_id);

    // 调用服务层获取EML数据
    match services.email.download_email_eml(&request.email_id).await {
        Ok(eml_data) => {
            // 获取邮件详情以获取主题作为文件名
            let filename = match services.email.get_email_detail(&request.email_id).await {
                Ok(email) => format!("{}.eml", email.subject.replace(" ", "_")),
                Err(_) => format!("email_{}.eml", request.email_id),
            };

            // 构建成功响应
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "message/rfc822")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(eml_data))
                .unwrap()
        }
        Err(e) => {
            // 构建错误响应
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(format!("下载邮件失败: {}", e)))
                .unwrap()
        }
    }
}

/// POST请求数据结构体 - 下载附件
#[derive(serde::Deserialize)]
pub struct DownloadAttachmentRequest {
    /// 附件ID
    pub attachment_id: String,
    /// 文件路径
    pub file_path: String,
}

/// 下载邮件附件
pub async fn download_attachment(
    State(services): State<AppServices>,
    Json(request): Json<DownloadAttachmentRequest>,
) -> Response<Body> {
    info!(
        "路由: 下载附件: attachment_id={}, file_path={}",
        request.attachment_id, request.file_path
    );

    // URL解码文件路径
    let decoded_path = match urlencoding::decode(&request.file_path) {
        Ok(path) => path.into_owned(),
        Err(_) => request.file_path.clone(),
    };

    // 调用服务层获取附件数据
    match services
        .email
        .download_attachment(&request.attachment_id, &decoded_path)
        .await
    {
        Ok((attachment_data, filename, content_type)) => {
            // 构建成功响应
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(attachment_data))
                .unwrap()
        }
        Err(e) => {
            // 构建错误响应
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(format!("下载附件失败: {}", e)))
                .unwrap()
        }
    }
}
