use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 变化方向枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeDirection {
    /// 增加
    Increase,
    /// 下降
    Decrease,
    /// 持平
    Unchanged,
}

/// 统计数据查询过滤条件 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsFilter {
    /// 开始时间
    pub start_time: DateTime<Utc>,
    /// 结束时间
    pub end_time: DateTime<Utc>,
    /// 查询模块，用于区分查询哪个板块
    pub module: String,
    /// 额外参数，预留字段，用于未来扩展
    pub extras: HashMap<String, String>,
}

/// 趋势数据点 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    /// 命中邮件数
    pub hit_emails: u64,
    /// 命中情报数
    pub hit_intelligence: u64,
}

/// 类型1：基础趋势数据项 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicStatisticsItem {
    /// 名称
    pub title: String,
    /// 当前周期总数
    pub current_total: u64,
    /// 上一周期总数
    pub previous_total: u64,
    /// 变化方向
    pub change_direction: ChangeDirection,
    /// 变化绝对值
    pub change_value: u64,
    /// 趋势图横轴数据（时间点）
    pub trend_x: Vec<String>,
    /// 趋势图纵轴数据（数值）
    pub trend_y: Vec<u64>,
}

/// 类型2：组织统计数据项 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatisticsItem {
    /// 名称
    pub title: String,
    /// 总数
    pub total_count: u64,
    /// 黑产组织数量
    pub black_count: u64,
    /// APT账号数量
    pub apt_count: u64,
}

/// 类型3：自定义情报命中统计数据项 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelHitStatisticsItem {
    /// 名称
    pub title: String,
    /// 自定义情报命中数量
    pub hit_custom_intel_count: u64,
    /// 自定义规则总数
    pub total_custom_intel_count: u64,
}

/// 类型4：趋势图数据项 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendChartItem {
    /// 横坐标（时间点）
    pub x_axis: Vec<String>,
    /// 纵坐标（数据点）
    pub y_axis: Vec<TrendPoint>,
}

/// 统计数据项 - 领域模型（兼容原有结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsItem {
    /// 名称
    pub title: String,
    /// 当前周期总数
    pub current_total: u64,
    /// 上一周期总数
    pub previous_total: u64,
    /// 变化方向
    pub change_direction: ChangeDirection,
    /// 变化绝对值
    pub change_value: u64,
    /// 趋势图横轴数据（时间点），可选
    pub trend_x: Option<Vec<String>>,
    /// 趋势图纵轴数据（数值），可选
    pub trend_y: Option<Vec<u64>>,
    /// APT数量，可选
    pub apt_count: Option<u64>,
    /// 黑产组织数量，可选
    pub black_count: Option<u64>,
    /// 自定义情报总数量，可选
    pub custom_count: Option<u64>,
}

/// 统计数据结果 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatisticsResult {
    /// 基础趋势统计数据
    BasicStats(Vec<BasicStatisticsItem>),
    /// 组织统计数据
    OrgStats(Vec<OrganizationStatisticsItem>),
    /// 情报命中统计数据
    IntelHitStats(Vec<IntelHitStatisticsItem>),
    /// 趋势图数据
    TrendChart(TrendChartItem),
} 