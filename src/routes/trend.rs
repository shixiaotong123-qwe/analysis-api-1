use axum::{
    extract::Json,
    http::StatusCode,
    Extension,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::server::AppState;

/// 趋势查询参数
#[derive(Debug, Deserialize)]
pub struct TrendQuery {
    /// 开始时间
    start_time: DateTime<Utc>,
    /// 结束时间
    end_time: DateTime<Utc>,
}

/// 趋势数据点
#[derive(Debug, Serialize)]
pub struct TrendPoint {
    /// 命中邮件数
    hit_emails: u64,
    /// 命中情报数
    hit_intelligence: u64,
}

/// 趋势响应
#[derive(Debug, Serialize)]
pub struct TrendResponse {
    /// 状态码
    code: u32,
    /// 数据
    data: TrendData,
}

/// 趋势数据
#[derive(Debug, Serialize)]
pub struct TrendData {
    /// 横坐标（时间点）
    x_axis: Vec<String>,
    /// 纵坐标（数据点）
    y_axis: Vec<TrendPoint>,
}

/// 查询命中邮件趋势
pub async fn query_hit_trend(
    Extension(_state): Extension<AppState>,
    Json(query): Json<TrendQuery>,
) -> Result<Json<TrendResponse>, (StatusCode, String)> {
    info!(
        "查询命中邮件趋势: start_time={}, end_time={}", 
        query.start_time, 
        query.end_time
    );

    // 生成模拟数据
    let response = TrendResponse {
        code: 200,
        data: TrendData {
            x_axis: vec![
                "2024-03-01".to_string(),
                "2024-03-02".to_string(),
                "2024-03-03".to_string(),
                "2024-03-04".to_string(),
                "2024-03-05".to_string(),
            ],
            y_axis: vec![
                TrendPoint {
                    hit_emails: 308,
                    hit_intelligence: 150,
                },
                TrendPoint {
                    hit_emails: 320,
                    hit_intelligence: 165,
                },
                TrendPoint {
                    hit_emails: 350,
                    hit_intelligence: 180,
                },
                TrendPoint {
                    hit_emails: 370,
                    hit_intelligence: 190,
                },
                TrendPoint {
                    hit_emails: 385,
                    hit_intelligence: 200,
                },
            ]
        }
    };

    Ok(Json(response))
} 