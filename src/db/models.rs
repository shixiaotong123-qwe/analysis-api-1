use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 情报属性枚举类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    Domain = 1,
    Url = 2,
    EmailAddress = 3,
    Ipv4 = 4,
    Md5 = 5,
    UrlDomain = 6,
    EmailDomain = 7,
    Sha256 = 8,
}

/// 情报紧急程度枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    High = 1,
    Medium = 2,
    Low = 3,
}

/// 情报来源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Local = 1,
    Cloud = 2,
}

/// 父文件来源类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParentSourceType {
    Email = 1,
    File = 2,
    EmailHeader = 3,
    EmailBody = 4,
    QrCode = 5,
    Text = 6,
    Url = 7,
    Smtp = 8,
}

/// 处置动作枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Accept = 1,
    Discard = 2,
    Reject = 3,
    Quarantine = 4,
}

/// 警报情报模型 - 对应alert_intelligence表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertIntelligence {
    /// 日志ID
    pub id: u64,
    /// 关联邮件的ID
    pub mail_id: u64,
    /// 邮件检测时间
    pub timestamp: DateTime<Utc>,
    /// 情报记录唯一标识符，使用UUID格式
    pub intelligence_id: Uuid,
    /// 情报描述，详细说明该情报的上下文和威胁情况
    pub description: String,
    /// 情报来源行业，采用GB/T 4754-2017行业分类标准，JSON数组格式
    pub source_industry: String,
    /// 首次发现时间，首次发现该情报的活动的时间
    pub first_discovered_time: DateTime<Utc>,
    /// 最后活跃时间，最后发现该情报活动的时间
    pub last_active_time: DateTime<Utc>,
    /// 情报更新时间，威胁情报最近的更新时间
    pub intelligence_update_time: DateTime<Utc>,
    /// 情报过期时间，威胁情报的过期时间
    pub intelligence_expiration_time: DateTime<Utc>,
    /// 情报属性，枚举类型，如domain、url、email-address等
    pub attribute: AttributeType,
    /// 情报分类，如钓鱼欺诈、傀儡账号、无效账号等
    pub intelligence_type: String,
    /// 情报紧急程度，分为高中低
    pub urgency: UrgencyLevel,
    /// 情报内容，存储实际的IOC值，如域名、URL、邮箱地址等
    pub value: String,
    /// 情报匹配模式，包含string和pcre两种方式
    pub pattern: String,
    /// 类型特定信息，根据attribute类型不同存储不同的结构化数据，如DNS信息、ICP备案等
    pub info: String,
    /// 攻击组织信息，包含攻击组织名称、描述、类型等详细信息
    pub threat_actor: String,
    /// 联防联控信息，记录命中单位名称、归属行业、命中数量等
    pub joint_prevention_and_control: String,
    /// 显示收件人名称
    pub display_to_name: String,
    /// 显示收件人地址
    pub display_to_address: String,
    /// 显示收件人账号
    pub display_to_account: String,
    /// 显示收件人域名
    pub display_to_domain: String,
    /// 逻辑删除标记，1表示已删除
    pub is_deleted: u8,
    /// 记录最后更新时间
    pub updated_at: DateTime<Utc>,
    /// 区分本地情报or云端情报
    pub source: SourceType,
    /// 父文件来源ID
    pub source_id: u64,
    /// 父文件类型
    pub source_mime_type: String,
    /// 父文件来源类型
    pub parent_source: ParentSourceType,
    /// 扫描耗时(微秒)
    pub scan_time_us: u64,
}

impl Row for AlertIntelligence {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "id", "mail_id", "timestamp", "intelligence_id", "description", 
        "source_industry", "first_discovered_time", "last_active_time", 
        "intelligence_update_time", "intelligence_expiration_time", 
        "attribute", "intelligence_type", "urgency", "value", "pattern", 
        "info", "threat_actor", "joint_prevention_and_control", 
        "display_to_name", "display_to_address", "display_to_account", 
        "display_to_domain", "is_deleted", "updated_at", "source", 
        "source_id", "source_mime_type", "parent_source", "scan_time_us"
    ];
}

/// 邮件信息模型 - 对应data_mail_info表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMailInfo {
    /// 邮件唯一ID
    pub id: u64,
    /// 处置动作：accept(接受), discard(丢弃), reject(拒绝), quarantine(隔离)
    pub action: ActionType,
    /// 邮件检测时间
    pub timestamp: DateTime<Utc>,
    /// 邮件发信时间（header中的Date）
    pub send_time: DateTime<Utc>,
    /// 邮件主题(header中的subject）
    pub subject: String,
    /// 密送收件人昵称
    pub bcc_name: String,
    /// 完整密送邮箱地址
    pub bcc_email: String,
    /// 密送邮箱账号部分（@前的字符）
    pub bcc_email_account: String,
    /// 密送邮箱完整域名
    pub bcc_email_domain: String,
    /// 显示发件人（header中的from）
    pub display_from: String,
    /// 显示收件人昵称
    pub display_to_name: String,
    /// 显示收件人完整邮箱地址
    pub display_to_address: String,
    /// 显示收件人邮箱账号部分（@前的字符）
    pub display_to_account: String,
    /// 显示收件人邮箱完整域名
    pub display_to_domain: String,
    /// 配置信息
    pub config: String,
    /// 认证用户名
    pub sasl_login: String,
    /// 认证方法
    pub sasl_method: String,
    /// 客户端IP
    pub client_ip: String,
    /// 客户端ptr
    pub client_ptr: String,
    /// 客户端端口
    pub client_port: i32,
    /// 客户端hello
    pub client_helo: String,
    /// 客户端活跃链接数
    pub client_active_connections: i32,
    /// 发件人昵称
    pub client_envelope_from_name: String,
    /// 发件人完整邮箱地址
    pub client_envelope_from_address: String,
    /// 发件人邮箱账号部分（@前的字符）
    pub client_envelope_from_account: String,
    /// 发件人邮箱完整域名
    pub client_envelope_from_domain: String,
    /// 收件人昵称
    pub client_envelope_to_name: String,
    /// 收件人完整邮箱地址
    pub client_envelope_to_address: String,
    /// 收件人邮箱账号部分（@前的字符）
    pub client_envelope_to_account: String,
    /// 收件人邮箱完整域名
    pub client_envelope_to_domain: String,
    /// 传输层安全协议信息
    pub tls: String,
    /// 服务器信息
    pub server: String,
    /// 协议信息
    pub protocol_version: String,
    /// 文本内容
    pub text_body: String,
    /// HTML内容
    pub html_body: String,
    /// 解构模块
    pub deconstruction_modules: String,
    /// 检测模块
    pub detection_modules: String,
    /// 文件SHA1哈希值
    pub hash_sha1: String,
    /// 文件SHA256哈希值
    pub hash_sha256: String,
    /// MD5哈希值
    pub hash_md5: String,
    /// 邮件方向
    pub direction: String,
    /// 邮件协议检测
    pub protocol_check: String,
    /// 提取密码
    pub extract_password: String,
}

impl Row for DataMailInfo {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "id", "action", "timestamp", "send_time", "subject", 
        "bcc_name", "bcc_email", "bcc_email_account", "bcc_email_domain",
        "display_from", "display_to_name", "display_to_address", 
        "display_to_account", "display_to_domain", "config",
        "sasl_login", "sasl_method", "client_ip", "client_ptr",
        "client_port", "client_helo", "client_active_connections",
        "client_envelope_from_name", "client_envelope_from_address",
        "client_envelope_from_account", "client_envelope_from_domain",
        "client_envelope_to_name", "client_envelope_to_address",
        "client_envelope_to_account", "client_envelope_to_domain",
        "tls", "server", "protocol_version", "text_body", "html_body",
        "deconstruction_modules", "detection_modules", "hash_sha1",
        "hash_sha256", "hash_md5", "direction", "protocol_check",
        "extract_password"
    ];
}

/// 查询计数结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountResult {
    /// 计数值
    pub count: u64,
}

impl Row for CountResult {
    const COLUMN_NAMES: &'static [&'static str] = &["count"];
}

/// 用户事件模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    /// 事件ID
    pub event_id: Uuid,
    /// 用户ID
    pub user_id: u64,
    /// 事件类型
    pub event_type: String,
    /// 事件数据（JSON格式）
    pub payload: String,
    /// 事件时间
    pub timestamp: DateTime<Utc>,
    /// 用户IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
}

impl Row for UserEvent {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "event_id", "user_id", "event_type", "payload", 
        "timestamp", "ip_address", "user_agent"
    ];
}

/// 分析结果模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// 结果ID
    pub result_id: Uuid,
    /// 分析名称
    pub analysis_name: String,
    /// 结果数据（JSON格式）
    pub result_data: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 分析参数（JSON格式）
    pub parameters: Option<String>,
}

impl AnalysisResult {
    // 注释未使用的常量
    // const FIELD_NAMES: &'static [&'static str] = &[
    //     "result_id", "analysis_name", "result_data", 
    //     "created_at", "updated_at", "parameters"
    // ];
} 