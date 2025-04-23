use chrono::{DateTime, Utc};

/// 趋势数据点 - 领域模型
#[derive(Debug, Clone)]
pub struct TrendPoint {
    /// 命中邮件数
    pub hit_emails: u64,
    /// 命中情报数
    pub hit_intelligence: u64,
}

/// 趋势数据 - 领域模型
#[derive(Debug, Clone)]
pub struct Trend {
    /// 查询开始时间
    pub start_time: DateTime<Utc>,
    /// 查询结束时间
    pub end_time: DateTime<Utc>,
    /// 横坐标（时间点）
    pub x_axis: Vec<String>,
    /// 纵坐标（数据点）
    pub y_axis: Vec<TrendPoint>,
} 