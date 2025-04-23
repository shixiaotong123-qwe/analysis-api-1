use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::domain::intelligence::{
    SourceType, IntelligenceType, StatusKey, SortField, SortOrder, 
    Intelligence, IntelligenceStatus, BasicInfo, IndustryDistribution
};

/// 情报查询请求参数 - API模型
#[derive(Debug, Deserialize)]
pub struct IntelligenceQueryParams {
    /// 开始时间
    #[serde(rename = "start_time")]
    pub start_time: Option<DateTime<Utc>>,
    
    /// 结束时间
    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,
    
    /// 情报来源
    #[serde(rename = "source")]
    pub source: Option<SourceType>,
    
    /// 情报类型
    /// 格式为: { "account": [], "domain": [], "url": [], "file": [] }
    /// 每种类型都是可选的，值为字符串数组
    #[serde(rename = "intelligence_type")]
    pub intelligence_type: Option<HashMap<IntelligenceType, Vec<String>>>,
    
    /// 处置状态
    /// 格式为: { "isWhite": false, "isBlack": false, "isReport": false }
    /// - isWhite: true 表示已加入白名单
    /// - isBlack: true 表示已加入黑名单
    /// - isReport: true 表示已上报
    #[serde(rename = "status", default = "default_status")]
    pub status: HashMap<StatusKey, bool>,

    /// 过滤值，用于模糊搜索情报值
    #[serde(default)]
    pub filter: Option<String>,

    /// 排序字段
    #[serde(default = "default_sort_field")]
    pub sort_by: SortField,

    /// 排序方向
    #[serde(default = "default_sort_order")]
    pub sort_order: SortOrder,
    
    /// 分页大小
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    
    /// 页码
    #[serde(default)]
    pub page: usize,
}

/// 情报列表响应项 - API模型
#[derive(Debug, Serialize)]
pub struct IntelligenceListItem {
    /// 情报日志ID
    pub id: Uuid,
    /// 情报ID
    pub intelligence_id: Uuid,
    /// 情报值
    pub value: String,
    /// 情报描述
    pub description: String,
    /// 情报主类型
    pub intelligence_type: IntelligenceType, 
    /// 情报子类型
    pub sub_type: String,
    /// 情报来源
    pub source: String,
    /// 紧急程度
    pub urgency: String,
    /// 命中邮件
    pub hit_emails: i32,
    /// 影响用户
    pub impact_users: i32,
    /// 首次发现时间
    pub first_found_time: DateTime<Utc>,
    /// 最新命中时间
    pub latest_hits_time: DateTime<Utc>,
    /// 处置状态
    pub status: IntelligenceStatus,
    /// 基本信息
    pub basic_info: BasicInfo,  
    /// 贡献单位
    pub contribution_unit: i32,
    /// 命中行业分布
    pub industry_distribution: Vec<IndustryDistribution>,
}

// 从领域模型转换为API模型
impl From<Intelligence> for IntelligenceListItem {
    fn from(intel: Intelligence) -> Self {
        Self {
            id: intel.id,
            intelligence_id: intel.intelligence_id,
            value: intel.value,
            description: intel.description,
            intelligence_type: intel.intelligence_type,
            sub_type: intel.sub_type,
            source: intel.source,
            urgency: intel.urgency,
            hit_emails: intel.hit_emails,
            impact_users: intel.impact_users,
            first_found_time: intel.first_found_time,
            latest_hits_time: intel.latest_hits_time,
            status: intel.status,
            basic_info: intel.basic_info,
            contribution_unit: intel.contribution_unit,
            industry_distribution: intel.industry_distribution,
        }
    }
}

/// 情报列表响应 - API模型
#[derive(Debug, Serialize)]
pub struct IntelligenceListResponse {
    /// 状态码
    pub code: u32,
    /// 情报列表
    pub data: Vec<IntelligenceListItem>,
    /// 总记录数
    pub total: u64
}

/// 默认分页大小
fn default_page_size() -> usize {
    10
}

/// 默认处置状态
fn default_status() -> HashMap<StatusKey, bool> {
    let mut status = HashMap::new();
    status.insert(StatusKey::IsWhite, false);
    status.insert(StatusKey::IsBlack, false);
    status.insert(StatusKey::IsReport, false);
    status
}

/// 默认排序字段
fn default_sort_field() -> SortField {
    SortField::LatestHitsTime
}

/// 默认排序方向
fn default_sort_order() -> SortOrder {
    SortOrder::Desc
} 