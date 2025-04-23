use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 情报来源类型
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum SourceType {
    /// 本地情报
    Local,
    /// 云端情报
    Cloud,
}

/// 情报类型枚举
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum IntelligenceType {
    /// 账号情报
    Account,
    /// 域名情报
    Domain,
    /// URL情报
    Url,
    /// 文件情报
    File,
}

/// 处置状态键名
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum StatusKey {
    /// 是否已加入白名单
    IsWhite,
    /// 是否已加入黑名单
    IsBlack,
    /// 是否已上报
    IsReport,
}

/// 紧急程度
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Urgency {
    /// 高
    High,
    /// 中
    Medium,
    /// 低
    Low, 
}

/// 排序字段
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    /// 最新命中时间
    LatestHitsTime,
    /// 命中邮件数
    HitEmails,
    /// 影响用户数
    ImpactUsers,
}

/// 排序方向
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 情报查询过滤条件 - 领域模型
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntelligenceFilter {
    /// 开始时间
    pub start_time: Option<DateTime<Utc>>,
    
    /// 结束时间
    pub end_time: Option<DateTime<Utc>>,
    
    /// 情报来源
    pub source: Option<SourceType>,
    
    /// 情报类型
    pub intelligence_type: Option<HashMap<IntelligenceType, Vec<String>>>,
    
    /// 处置状态
    pub status: HashMap<StatusKey, bool>,

    /// 过滤值，用于模糊搜索情报值
    pub filter: Option<String>,

    /// 排序字段
    pub sort_by: SortField,

    /// 排序方向
    pub sort_order: SortOrder,
    
    /// 分页大小
    pub page_size: usize,
    
    /// 页码
    pub page: usize,
}

/// 情报处置状态 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceStatus {
    /// 是否已加入白名单
    pub is_white: bool,
    /// 是否已加入黑名单
    pub is_black: bool,
    /// 是否已上报
    pub is_report: bool,
}

/// 基本信息 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    /// 文件名
    pub file_name: Option<String>,
    /// 文件路径
    pub file_path: Option<String>,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 文件类型
    pub file_type: Option<String>,
}

/// 命中行业分布 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryDistribution {
    /// 行业名称
    pub industry_name: String,
    /// 命中百分比
    pub hit_percentage: f32,
}

/// 情报项 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intelligence {
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