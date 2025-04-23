// 服务层模块入口
// 导出所有服务

pub mod statistics_service;
pub mod email_service;
pub mod intelligence_service;
pub mod timeline_service;

// 公开服务结构体
pub use statistics_service::StatisticsService;
pub use email_service::EmailService;
pub use intelligence_service::IntelligenceService;
pub use timeline_service::TimelineService;

use std::sync::Arc;
use crate::db::ClickHouseClient;

// 服务集合结构体，用于依赖注入
#[derive(Clone)]
pub struct AppServices {
    pub statistics: StatisticsService,
    pub email: EmailService,
    pub intelligence: IntelligenceService,
    pub timeline: TimelineService,
}

impl AppServices {
    pub fn new(db_client: Option<Arc<ClickHouseClient>>) -> Self {
        Self {
            statistics: StatisticsService::new(db_client.clone()),
            email: EmailService::new(db_client.clone()),
            intelligence: IntelligenceService::new(db_client.clone()),
            timeline: TimelineService::new(db_client.clone()),
        }
    }
} 