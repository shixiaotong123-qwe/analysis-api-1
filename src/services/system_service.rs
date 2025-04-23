use chrono::Utc;
use tracing::info;
use serde::Serialize;

/// 系统时间数据
#[derive(Debug, Clone, Serialize)]
pub struct SystemTimeData {
    /// 时间戳（毫秒）
    pub timestamp: i64,
}

/// 系统服务
#[derive(Clone)]
pub struct SystemService {}

impl SystemService {
    /// 创建新的系统服务实例
    pub fn new() -> Self {
        Self {}
    }

    /// 获取系统时间
    pub fn get_system_time(&self) -> SystemTimeData {
        info!("系统服务: 获取系统时间");
        
        let now = Utc::now();
        SystemTimeData {
            timestamp: now.timestamp_millis(),
        }
    }
} 