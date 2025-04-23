use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// 邮件附件信息 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
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

/// URL信息 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    /// URL ID
    pub id: String,
    /// URL地址
    pub url: String,
    /// URL路径
    pub path: String,
}

/// 邮件信息 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
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
    pub attachments: Vec<Attachment>,
    /// URL列表
    pub urls: Vec<Url>,
    /// 邮件内容
    pub content: String,
    /// 邮件状态
    pub status: String,
    /// 邮件源代码
    pub source_code: String,
}

/// 邮件查询过滤条件 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailFilter {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 情报ID
    pub intelligence_id: String,
    /// 邮件状态
    pub status: Option<String>,
    /// 页码
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
} 