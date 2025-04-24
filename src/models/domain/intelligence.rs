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

/// 账号子类型
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AccountSubType {
    /// 傀儡账号
    Puppet,
    /// 无效账号
    Invalid,
    /// 伪造账号
    Fake,
    /// 品牌仿冒账号
    BrandImitation,
    /// 匿名账号
    Anonymous,
    /// 滥用账号
    Abuse,
    /// APT账号
    Apt,
    /// 黑灰产账号
    BlackIndustry,
}

impl AsRef<str> for AccountSubType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Puppet => "傀儡账号",
            Self::Invalid => "无效账号",
            Self::Fake => "伪造账号",
            Self::BrandImitation => "品牌仿冒账号",
            Self::Anonymous => "匿名账号", 
            Self::Abuse => "滥用账号",
            Self::Apt => "APT账号",
            Self::BlackIndustry => "黑灰产账号",
        }
    }
}

/// 域名子类型
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum DomainSubType {
    /// 品牌仿冒域名
    BrandImitation,
    /// 攻击者注册的域名
    AttackerRegistered,
    /// 匿名邮箱域名
    AnonymousEmail,
    /// 无效域名
    Invalid,
}

impl AsRef<str> for DomainSubType {
    fn as_ref(&self) -> &str {
        match self {
            Self::BrandImitation => "品牌仿冒域名",
            Self::AttackerRegistered => "攻击者注册的域名",
            Self::AnonymousEmail => "匿名邮箱域名",
            Self::Invalid => "无效域名",
        }
    }
}

/// URL子类型
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum UrlSubType {
    /// 钓鱼欺诈
    Phishing,
    /// 品牌仿冒
    BrandImitation,
    /// 诱导下载
    InducedDownload,
}

impl AsRef<str> for UrlSubType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Phishing => "钓鱼欺诈",
            Self::BrandImitation => "品牌仿冒",
            Self::InducedDownload => "诱导下载",
        }
    }
}

/// 文件子类型
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FileSubType {
    /// 恶意文件HASH
    MaliciousHash,
}

impl AsRef<str> for FileSubType {
    fn as_ref(&self) -> &str {
        match self {
            Self::MaliciousHash => "恶意文件HASH",
        }
    }
}

/// 处置状态键名
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum StatusKey {
    /// 不看已加入白名单
    IgnoreWhite,
    /// 不看已加入黑名单
    IgnoreBlack,
    /// 不看已上报
    IgnoreReported,
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
    pub start_time: DateTime<Utc>,
    
    /// 结束时间
    pub end_time: DateTime<Utc>,
    
    /// 情报来源，可选多个来源类型
    pub sources: Option<Vec<SourceType>>,
    
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
    /// 不看已加入白名单
    pub ignore_white: bool,
    /// 不看已加入黑名单
    pub ignore_black: bool,
    /// 不看已上报
    pub ignore_reported: bool,
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
    /// 受攻击行业
    pub attacked_industry: i32,
    /// 贡献行业
    pub contribution_industry: String,
    /// 命中行业分布
    pub industry_distribution: Vec<IndustryDistribution>,
} 