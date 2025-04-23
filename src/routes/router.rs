use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::server::AppState;

/// 构建应用路由
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(super::hello))
        // 添加POST方式的情报查询
        .route("/intelligence/list", post(super::list_intelligence))
        // 添加POST方式的关联邮件查询
        .route("/intelligence/related-emails", post(super::query_related_emails))
        // 添加POST方式的攻击时间线查询
        .route("/intelligence/timeline", post(super::query_timeline))
        // 添加POST方式的统计数据查询
        .route("/intelligence/statistics", post(super::query_statistics))
        // 添加POST方式的命中邮件趋势查询
        .route("/intelligence/hit-emails-trend", post(super::query_hit_trend))
        // 添加POST方式的邮件EML下载
        .route("/email/download-eml", post(super::download_email_eml))
        // 添加POST方式的附件下载
        .route("/attachment/download", post(super::download_attachment))
        // 添加应用状态
        .with_state(state.services)
        // 添加tracing中间件
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO)),
        )
} 