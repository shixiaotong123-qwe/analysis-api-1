use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 情报操作响应 - API模型
#[derive(Debug, Serialize)]
pub struct ActionResponse {
    /// 状态码
    pub code: u32,
    /// 数据
    pub data: ActionResponseData,
}

/// 情报操作响应数据 - API模型
#[derive(Debug, Serialize)]
pub struct ActionResponseData {
    /// 消息
    pub msg: String,
}

/// 加白请求参数 - API模型
#[derive(Debug, Deserialize)]
pub struct WhitelistRequest {
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报值
    pub value: String,
    /// 是否放行隔离区相关邮件
    pub release_quarantine: bool,
}

/// 加黑请求参数 - API模型
#[derive(Debug, Deserialize)]
pub struct BlacklistRequest {
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报值
    pub value: String,
}

/// 上报请求参数 - API模型
#[derive(Debug, Deserialize)]
pub struct ReportRequest {
    /// 情报日志ID
    pub id: Uuid,
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 为什么认为是误报
    pub reason: String,
    /// 是否提供误报邮件
    pub provide_sample_email: bool,
} 