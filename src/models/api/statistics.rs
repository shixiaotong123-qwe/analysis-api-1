use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::models::domain::statistics::{ChangeDirection, StatisticsItem};

/// 统计数据查询参数
#[derive(Debug, Deserialize)]
pub struct StatisticsQuery {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
}

/// 变化方向枚举 - API模型
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeDirectionResponse {
    /// 增加
    Increase,
    /// 下降
    Decrease,
    /// 持平
    Unchanged,
}

// 从领域模型转换为API模型
impl From<ChangeDirection> for ChangeDirectionResponse {
    fn from(direction: ChangeDirection) -> Self {
        match direction {
            ChangeDirection::Increase => ChangeDirectionResponse::Increase,
            ChangeDirection::Decrease => ChangeDirectionResponse::Decrease,
            ChangeDirection::Unchanged => ChangeDirectionResponse::Unchanged,
        }
    }
}

/// 统计数据项 - API模型
#[derive(Debug, Serialize)]
pub struct StatisticsItemResponse {
    /// 名称
    pub title: String,
    /// 当前周期总数
    pub current_total: u64,
    /// 上一周期总数
    pub previous_total: u64,
    /// 变化方向
    pub change_direction: ChangeDirectionResponse,
    /// 变化绝对值
    pub change_value: u64,
    /// 趋势图横轴数据（时间点），可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_x: Option<Vec<String>>,
    /// 趋势图纵轴数据（数值），可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_y: Option<Vec<u64>>,
    /// APT数量，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apt_count: Option<u64>,
    /// 黑产组织数量，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_count: Option<u64>,
    /// 自定义情报总数量，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_count: Option<u64>,
}

// 从领域模型转换为API模型
impl From<StatisticsItem> for StatisticsItemResponse {
    fn from(item: StatisticsItem) -> Self {
        Self {
            title: item.title,
            current_total: item.current_total,
            previous_total: item.previous_total,
            change_direction: item.change_direction.into(),
            change_value: item.change_value,
            trend_x: item.trend_x,
            trend_y: item.trend_y,
            apt_count: item.apt_count,
            black_count: item.black_count,
            custom_count: item.custom_count,
        }
    }
}

/// 统计数据响应
#[derive(Debug, Serialize)]
pub struct StatisticsResponse {
    /// 状态码
    pub code: u32,
    /// 统计数据列表
    pub data: Vec<StatisticsItemResponse>,
} 