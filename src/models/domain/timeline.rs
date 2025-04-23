use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 邮件信息 - 领域模型
#[derive(Debug, Clone)]
pub struct TimelineEmail {
    /// 邮件ID
    pub mail_id: u64,
    /// 邮件时间
    pub timestamp: DateTime<Utc>,
    /// 邮件状态
    pub status: String,
    /// 发件人
    pub sender: String,
    /// 收件人
    pub recipient: Vec<String>,
}

/// 攻击时间线数据 - 领域模型
#[derive(Debug, Clone)]
pub struct Timeline {
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报首次发现时间
    pub first_found_time: DateTime<Utc>,
    /// 情报来源
    pub source: String,
    /// 相关邮件列表
    pub emails: Vec<TimelineEmail>,
} 