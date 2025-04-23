//! 数据库模块

pub mod models;
// 保留repository模块但标记为废弃
// pub mod repository;
pub mod clickhouse;

// 导出主要类型
pub use models::{
    UserEvent, AnalysisResult, CountResult,
};
// 移除repository的导出
// pub use repository::{
//     UserEventRepository, AnalysisResultRepository,
//     ClickHouseUserEventRepository, ClickHouseAnalysisResultRepository,
//     InMemoryUserEventRepository, InMemoryAnalysisResultRepository,
// };
pub use clickhouse::ClickHouseClient;

/// 数据库配置
#[derive(Clone, Debug)]
pub struct DbConfig {
    /// 数据库URL
    pub url: String,
    /// 数据库名称
    pub database: String,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "http://192.168.2.225:8123".to_string(),
            username: Some("m01".to_string()),
            password: Some("Welcome1@2024".to_string()),
            database: "m01".to_string(),
        }
    }
}

/// 数据库错误类型
pub type DbError = anyhow::Error;
/// 数据库结果类型
pub type DbResult<T> = Result<T, DbError>; 