// 服务层模块入口
// 导出所有服务

pub mod system_service;
pub mod statistics_service;
pub mod email_service;
pub mod intelligence_service;
pub mod timeline_service;
pub mod trend_service;

// 公开服务结构体
pub use system_service::SystemService;
pub use statistics_service::StatisticsService;
pub use email_service::EmailService;
pub use intelligence_service::IntelligenceService;
pub use timeline_service::TimelineService;
pub use trend_service::TrendService;

use std::sync::Arc;
use crate::db::ClickHouseClient;

// 服务集合结构体，用于依赖注入
#[derive(Clone)]
pub struct AppServices {
    pub system: SystemService,
    pub statistics: StatisticsService,
    pub email: EmailService,
    pub intelligence: IntelligenceService,
    pub timeline: TimelineService,
    pub trend: TrendService,
}

impl AppServices {
    pub fn new(db_client: Option<Arc<ClickHouseClient>>) -> Self {
        Self {
            system: SystemService::new(),
            statistics: StatisticsService::new(db_client.clone()),
            email: EmailService::new(db_client.clone()),
            intelligence: IntelligenceService::new(db_client.clone()),
            timeline: TimelineService::new(db_client.clone()),
            trend: TrendService::new(db_client),
        }
    }
} 