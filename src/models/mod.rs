// 模型模块入口
// 导出所有子模块和主要类型

pub mod domain;  // 领域模型
pub mod api;     // API模型

// 重新导出常用类型，方便直接使用
pub use domain::statistics::{ChangeDirection, StatisticsItem};
pub use api::statistics::{StatisticsQuery, StatisticsResponse, StatisticsItemResponse};

// 导出邮件相关模型
pub use domain::email::{Email, Attachment, Url, EmailFilter};
pub use api::email::{RelatedEmailsQuery, RelatedEmailsResponse, EmailResponse};

// 导出情报相关模型
pub use domain::intelligence::{Intelligence, IntelligenceFilter, IntelligenceType, SourceType, StatusKey, SortField, SortOrder};
pub use api::intelligence::{IntelligenceQueryParams, IntelligenceListResponse, IntelligenceListItem};

// 导出时间线相关模型
pub use domain::timeline::{Timeline, TimelineEmail};
pub use api::timeline::{TimelineQuery, TimelineResponse, TimelineData, TimelineEmailResponse};

// 导出趋势相关模型
pub use domain::trend::{Trend, TrendPoint};
pub use api::trend::{TrendQuery, TrendResponse, TrendData, TrendPointResponse};
