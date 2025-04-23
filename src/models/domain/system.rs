use chrono::Utc;
use serde::Serialize;

/// 系统时间数据 - 领域模型
#[derive(Debug, Serialize, Clone)]
pub struct SystemTimeData {
    /// 时间戳（毫秒）
    pub timestamp: i64,
    /// 当前系统时间（ISO 8601格式）
    pub current_time: String,
    /// 时区信息
    pub timezone: String,
}

impl SystemTimeData {
    /// 从服务层数据创建领域模型
    pub fn from_service(service_data: crate::services::system_service::SystemTimeData) -> Self {
        let now = Utc::now();
        Self {
            timestamp: service_data.timestamp,
            current_time: now.to_rfc3339(),
            timezone: "UTC".to_string(),
        }
    }
} 