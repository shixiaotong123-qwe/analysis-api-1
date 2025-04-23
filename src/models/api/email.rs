use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::models::domain::email::{Email, Attachment, Url};

/// 关联邮件查询参数 - API模型
#[derive(Debug, Deserialize)]
pub struct RelatedEmailsQuery {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 情报ID
    pub intelligence_id: String,
    /// 邮件状态
    pub status: Option<String>,
    /// 页码
    pub page: Option<u32>,
    /// 每页大小
    pub page_size: Option<u32>,
}

/// 邮件附件信息 - API模型
#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    /// 附件ID
    pub id: String,
    /// 文件名
    pub filename: String,
    /// 文件路径
    pub file_path: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 文件扩展名
    pub file_extension: String,
    /// 文件MD5值
    pub md5: String,
}

// 从领域模型转换
impl From<Attachment> for AttachmentResponse {
    fn from(attachment: Attachment) -> Self {
        Self {
            id: attachment.id,
            filename: attachment.filename,
            file_path: attachment.file_path,
            size: attachment.size,
            file_extension: attachment.file_extension,
            md5: attachment.md5,
        }
    }
}

/// URL信息 - API模型
#[derive(Debug, Serialize)]
pub struct UrlResponse {
    /// URL ID
    pub id: String,
    /// URL地址
    pub url: String,
    /// URL路径
    pub path: String,
}

// 从领域模型转换
impl From<Url> for UrlResponse {
    fn from(url: Url) -> Self {
        Self {
            id: url.id,
            url: url.url,
            path: url.path,
        }
    }
}

/// 邮件信息 - API模型
#[derive(Debug, Serialize)]
pub struct EmailResponse {
    /// 邮件ID
    pub id: String,
    /// 邮件时间
    pub timestamp: DateTime<Utc>,
    /// 主题
    pub subject: String,
    /// 发件人
    pub sender: String,
    /// 收件人
    pub recipients: Vec<String>,
    /// 附件列表
    pub attachments: Vec<AttachmentResponse>,
    /// URL列表
    pub urls: Vec<UrlResponse>,
    /// 邮件内容
    pub content: String,
    /// 邮件状态
    pub status: String,
    /// 邮件源代码
    pub source_code: String,
}

// 从领域模型转换
impl From<Email> for EmailResponse {
    fn from(email: Email) -> Self {
        Self {
            id: email.id,
            timestamp: email.timestamp,
            subject: email.subject,
            sender: email.sender,
            recipients: email.recipients,
            attachments: email.attachments.into_iter().map(AttachmentResponse::from).collect(),
            urls: email.urls.into_iter().map(UrlResponse::from).collect(),
            content: email.content,
            status: email.status,
            source_code: email.source_code,
        }
    }
}

/// 关联邮件查询响应 - API模型
#[derive(Debug, Serialize)]
pub struct RelatedEmailsResponse {
    /// 状态码
    pub code: u32,
    /// 总数
    pub total: u32,
    /// 邮件列表
    pub data: Vec<EmailResponse>,
} 