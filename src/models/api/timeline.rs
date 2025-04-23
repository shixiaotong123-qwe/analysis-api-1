use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::domain::timeline::{Timeline, TimelineEmail};

/// 攻击时间线查询参数 - API模型
#[derive(Debug, Deserialize)]
pub struct TimelineQuery {
    /// 情报ID
    pub intelligence_id: Uuid,
}

/// 邮件信息 - API模型
#[derive(Debug, Serialize)]
pub struct TimelineEmailResponse {
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

// 从领域模型转换为API模型
impl From<TimelineEmail> for TimelineEmailResponse {
    fn from(email: TimelineEmail) -> Self {
        Self {
            mail_id: email.mail_id,
            timestamp: email.timestamp,
            status: email.status,
            sender: email.sender,
            recipient: email.recipient,
        }
    }
}

/// 攻击时间线数据 - API模型
#[derive(Debug, Serialize)]
pub struct TimelineData {
    /// 情报首次发现时间
    pub first_found_time: DateTime<Utc>,
    /// 情报来源
    pub source: String,
    /// 相关邮件列表
    pub emails: Vec<TimelineEmailResponse>,
}

// 从领域模型转换为API模型
impl From<Timeline> for TimelineData {
    fn from(timeline: Timeline) -> Self {
        Self {
            first_found_time: timeline.first_found_time,
            source: timeline.source,
            emails: timeline.emails.into_iter().map(TimelineEmailResponse::from).collect(),
        }
    }
}

/// 攻击时间线响应 - API模型
#[derive(Debug, Serialize)]
pub struct TimelineResponse {
    /// 状态码
    pub code: u32,
    /// 数据
    pub data: TimelineData,
} 