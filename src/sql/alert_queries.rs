/// 创建alert_intelligence表的SQL
pub const CREATE_ALERT_INTELLIGENCE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS alert_intelligence (
    alert_id UUID,
    source String,
    severity String,
    title String,
    description String,
    created_at DateTime64(3, 'UTC'),
    updated_at DateTime64(3, 'UTC'),
    status String,
    tags Array(String),
    attributes String,
    related_assets String
) ENGINE = MergeTree()
ORDER BY (source, created_at)
"#;

/// 查询所有Alert记录，带分页
pub const SELECT_ALERTS_WITH_PAGINATION: &str = r#"
SELECT * FROM alert_intelligence 
ORDER BY created_at DESC 
LIMIT ? OFFSET ?
"#;

/// 查询Alert总数
pub const COUNT_ALERTS: &str = r#"
SELECT COUNT(*) as count FROM alert_intelligence
"#;

/// 按ID查询Alert
pub const SELECT_ALERT_BY_ID: &str = r#"
SELECT * FROM alert_intelligence WHERE alert_id = ?
"#;

/// 按状态查询Alert
pub const SELECT_ALERTS_BY_STATUS: &str = r#"
SELECT * FROM alert_intelligence WHERE status = ?
ORDER BY created_at DESC
"#;

/// 按严重性查询Alert
pub const SELECT_ALERTS_BY_SEVERITY: &str = r#"
SELECT * FROM alert_intelligence WHERE severity = ?
ORDER BY created_at DESC
"#;

/// 按来源查询Alert
pub const SELECT_ALERTS_BY_SOURCE: &str = r#"
SELECT * FROM alert_intelligence WHERE source = ?
ORDER BY created_at DESC
"#; 