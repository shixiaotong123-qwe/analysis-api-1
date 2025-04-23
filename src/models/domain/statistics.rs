use serde::{Serialize, Deserialize};

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

/// 统计数据项 - 领域模型
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