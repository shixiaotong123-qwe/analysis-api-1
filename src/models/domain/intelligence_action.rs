use uuid::Uuid;

/// 情报白名单操作 - 领域模型
#[derive(Debug, Clone)]
pub struct WhitelistAction {
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报值
    pub value: String,
    /// 是否放行隔离区相关邮件
    pub release_quarantine: bool,
}

/// 情报黑名单操作 - 领域模型
#[derive(Debug, Clone)]
pub struct BlacklistAction {
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报值
    pub value: String,
}

/// 情报上报操作 - 领域模型
#[derive(Debug, Clone)]
pub struct ReportAction {
    /// 情报日志ID
    pub id: Uuid,
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 为什么认为是误报
    pub reason: String,
    /// 是否提供误报邮件
    pub provide_sample_email: bool,
}

// 从API模型转换为领域模型的实现
impl From<crate::models::api::intelligence_action::WhitelistRequest> for WhitelistAction {
    fn from(request: crate::models::api::intelligence_action::WhitelistRequest) -> Self {
        Self {
            intelligence_id: request.intelligence_id,
            value: request.value,
            release_quarantine: request.release_quarantine,
        }
    }
}

impl From<crate::models::api::intelligence_action::BlacklistRequest> for BlacklistAction {
    fn from(request: crate::models::api::intelligence_action::BlacklistRequest) -> Self {
        Self {
            intelligence_id: request.intelligence_id,
            value: request.value,
        }
    }
}

impl From<crate::models::api::intelligence_action::ReportRequest> for ReportAction {
    fn from(request: crate::models::api::intelligence_action::ReportRequest) -> Self {
        Self {
            id: request.id,
            intelligence_id: request.intelligence_id,
            reason: request.reason,
            provide_sample_email: request.provide_sample_email,
        }
    }
} 