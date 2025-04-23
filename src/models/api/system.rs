use serde::Serialize;
use crate::models::domain::system::SystemTimeData;

/// 系统时间响应 - API模型
#[derive(Debug, Serialize)]
pub struct SystemTimeResponse {
    /// 状态码
    pub code: u32,
    /// 数据
    pub data: SystemTimeData,
}

// 创建响应的便捷方法
impl SystemTimeResponse {
    /// 创建成功响应
    pub fn success(data: SystemTimeData) -> Self {
        Self {
            code: 200,
            data,
        }
    }
} 