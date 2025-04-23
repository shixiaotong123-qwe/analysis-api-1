use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::models::domain::statistics::{
    ChangeDirection, BasicStatisticsItem, OrganizationStatisticsItem, 
    IntelHitStatisticsItem, TrendChartItem, TrendPoint, StatisticsResult
};

/// 统计数据查询参数
#[derive(Debug, Deserialize)]
pub struct StatisticsQuery {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 查询模块，用于区分查询哪个板块
    pub module: String,
    /// 额外参数，预留字段，用于未来扩展
    #[serde(default)]
    pub extras: HashMap<String, String>,
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

/// 趋势数据点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPointResponse {
    /// 命中邮件数
    pub hit_emails: u64,
    /// 命中情报数
    pub hit_intelligence: u64,
}

impl From<TrendPoint> for TrendPointResponse {
    fn from(point: TrendPoint) -> Self {
        Self {
            hit_emails: point.hit_emails,
            hit_intelligence: point.hit_intelligence,
        }
    }
}

/// 类型1：基础统计项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicStatisticsItemResponse {
    /// 名称
    pub title: String,
    /// 当前总数
    pub current_total: u64,
    /// 上一周期总数
    pub previous_total: u64,
    /// 变化方向，increase/decrease/unchanged
    pub change_direction: String,
    /// 变化绝对值
    pub change_value: u64,
    /// 趋势图横轴数据（时间点）
    pub trend_x: Vec<String>,
    /// 趋势图纵轴数据（数值）
    pub trend_y: Vec<u64>,
}

impl From<BasicStatisticsItem> for BasicStatisticsItemResponse {
    fn from(item: BasicStatisticsItem) -> Self {
        Self {
            title: item.title,
            current_total: item.current_total,
            previous_total: item.previous_total,
            change_direction: match item.change_direction {
                ChangeDirection::Increase => "increase".to_string(),
                ChangeDirection::Decrease => "decrease".to_string(),
                ChangeDirection::Unchanged => "unchanged".to_string(),
            },
            change_value: item.change_value,
            trend_x: item.trend_x,
            trend_y: item.trend_y,
        }
    }
}

/// 类型2：APT/黑产组织统计项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatisticsItemResponse {
    /// 名称
    pub title: String,
    /// 总数
    pub total_count: u64,
    /// 黑产组织数量
    pub black_count: u64,
    /// APT账号数量
    pub apt_count: u64,
}

impl From<OrganizationStatisticsItem> for OrganizationStatisticsItemResponse {
    fn from(item: OrganizationStatisticsItem) -> Self {
        Self {
            title: item.title,
            total_count: item.total_count,
            black_count: item.black_count,
            apt_count: item.apt_count,
        }
    }
}

/// 类型3：自定义情报命中统计项响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelHitStatisticsItemResponse {
    /// 名称
    pub title: String,
    /// 自定义情报命中数量
    pub hit_custom_intel_count: u64,
    /// 自定义规则总数
    pub total_custom_intel_count: u64,
}

impl From<IntelHitStatisticsItem> for IntelHitStatisticsItemResponse {
    fn from(item: IntelHitStatisticsItem) -> Self {
        Self {
            title: item.title,
            hit_custom_intel_count: item.hit_custom_intel_count,
            total_custom_intel_count: item.total_custom_intel_count,
        }
    }
}

/// 类型4：趋势图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendChartResponse {
    /// 横坐标（时间点）
    pub x_axis: Vec<String>,
    /// 纵坐标（数据点）
    pub y_axis: Vec<TrendPointResponse>,
}

impl From<TrendChartItem> for TrendChartResponse {
    fn from(item: TrendChartItem) -> Self {
        Self {
            x_axis: item.x_axis,
            y_axis: item.y_axis.into_iter().map(TrendPointResponse::from).collect(),
        }
    }
}

/// 统计响应数据 - 使用枚举封装不同类型的响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StatisticsResponseData {
    /// 基础统计数据
    #[serde(rename = "basic")]
    BasicStats(Vec<BasicStatisticsItemResponse>),
    /// 组织统计数据
    #[serde(rename = "organization")]
    OrgStats(Vec<OrganizationStatisticsItemResponse>),
    /// 情报命中统计数据
    #[serde(rename = "intel_hit")]
    IntelHitStats(Vec<IntelHitStatisticsItemResponse>),
    /// 趋势图数据
    #[serde(rename = "trend_chart")]
    TrendChart(TrendChartResponse),
}

impl From<StatisticsResult> for StatisticsResponseData {
    fn from(result: StatisticsResult) -> Self {
        match result {
            StatisticsResult::BasicStats(items) => {
                StatisticsResponseData::BasicStats(
                    items.into_iter().map(BasicStatisticsItemResponse::from).collect()
                )
            },
            StatisticsResult::OrgStats(items) => {
                StatisticsResponseData::OrgStats(
                    items.into_iter().map(OrganizationStatisticsItemResponse::from).collect()
                )
            },
            StatisticsResult::IntelHitStats(items) => {
                StatisticsResponseData::IntelHitStats(
                    items.into_iter().map(IntelHitStatisticsItemResponse::from).collect()
                )
            },
            StatisticsResult::TrendChart(item) => {
                StatisticsResponseData::TrendChart(TrendChartResponse::from(item))
            },
        }
    }
}

/// 统计查询请求 - API 模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsRequest {
    /// 开始时间，ISO 8601格式字符串
    pub start_time: DateTime<Utc>,
    /// 结束时间，ISO 8601格式字符串
    pub end_time: DateTime<Utc>,
    /// 查询模块，用于区分查询哪个板块
    pub module: String,
    /// 额外参数，预留字段，用于未来扩展
    #[serde(default)]
    pub extras: HashMap<String, String>,
}

/// 统计响应 - API 模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsResponse {
    /// 状态码，200表示成功
    pub code: u32,
    /// 响应数据
    pub data: StatisticsResponseData,
} 