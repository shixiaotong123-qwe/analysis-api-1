use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::domain::trend::{Trend, TrendPoint};

/// 趋势查询参数 - API模型
#[derive(Debug, Deserialize)]
pub struct TrendQuery {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
}

/// 趋势数据点 - API模型
#[derive(Debug, Serialize)]
pub struct TrendPointResponse {
    /// 命中邮件数
    pub hit_emails: u64,
    /// 命中情报数
    pub hit_intelligence: u64,
}

// 从领域模型转换为API模型
impl From<TrendPoint> for TrendPointResponse {
    fn from(point: TrendPoint) -> Self {
        Self {
            hit_emails: point.hit_emails,
            hit_intelligence: point.hit_intelligence,
        }
    }
}

/// 趋势数据 - API模型
#[derive(Debug, Serialize)]
pub struct TrendData {
    /// 横坐标（时间点）
    pub x_axis: Vec<String>,
    /// 纵坐标（数据点）
    pub y_axis: Vec<TrendPointResponse>,
}

// 从领域模型转换为API模型
impl From<Trend> for TrendData {
    fn from(trend: Trend) -> Self {
        Self {
            x_axis: trend.x_axis,
            y_axis: trend.y_axis.into_iter().map(TrendPointResponse::from).collect(),
        }
    }
}

/// 趋势响应 - API模型
#[derive(Debug, Serialize)]
pub struct TrendResponse {
    /// 状态码
    pub code: u32,
    /// 数据
    pub data: TrendData,
} 