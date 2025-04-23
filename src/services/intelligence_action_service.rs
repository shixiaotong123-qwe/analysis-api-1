use std::sync::Arc;
use anyhow::{Result, Context};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::db::ClickHouseClient;

/// 情报操作服务
#[derive(Clone)]
pub struct IntelligenceActionService {
    /// ClickHouse客户端，可选（可能运行在内存模式）
    db_client: Option<Arc<ClickHouseClient>>,
}

impl IntelligenceActionService {
    /// 创建新的情报操作服务实例
    pub fn new(db_client: Option<Arc<ClickHouseClient>>) -> Self {
        Self { db_client }
    }

    /// 将情报添加到白名单
    #[instrument(skip(self))]
    pub async fn add_to_whitelist(&self, intelligence_id: Uuid, value: String, release_quarantine: bool) -> Result<()> {
        info!(
            "情报操作服务: 添加情报到白名单，情报ID: {}, 情报值: {}, 放行隔离邮件: {}",
            intelligence_id, value, release_quarantine
        );

        // 检查情报是否存在
        self.check_intelligence_exists(intelligence_id).await?;

        // 如果有数据库连接，执行实际操作
        if let Some(client) = &self.db_client {
            info!("有数据库连接，将执行白名单操作");
            self.execute_whitelist_db(client, intelligence_id, value, release_quarantine).await
                .context("执行白名单数据库操作失败")?;
        } else {
            info!("无数据库连接，仅模拟白名单操作");
            // 在内存模式下模拟操作成功
        }

        // 如果需要放行隔离区邮件
        if release_quarantine {
            self.release_quarantined_emails(intelligence_id).await
                .context("放行隔离邮件失败")?;
        }

        Ok(())
    }

    /// 将情报添加到黑名单
    #[instrument(skip(self))]
    pub async fn add_to_blacklist(&self, intelligence_id: Uuid, value: String, _quarantine_emails: bool) -> Result<()> {
        info!(
            "情报操作服务: 添加情报到黑名单，情报ID: {}, 情报值: {}",
            intelligence_id, value
        );

        // 检查情报是否存在
        self.check_intelligence_exists(intelligence_id).await?;

        // 如果有数据库连接，执行实际操作
        if let Some(client) = &self.db_client {
            info!("有数据库连接，将执行黑名单操作");
            self.execute_blacklist_db(client, intelligence_id, value).await
                .context("执行黑名单数据库操作失败")?;
        } else {
            info!("无数据库连接，仅模拟黑名单操作");
            // 在内存模式下模拟操作成功
        }

        // 不再根据参数决定是否隔离相关邮件
        // 系统将自动隔离相关邮件
        self.quarantine_related_emails(intelligence_id).await
            .context("隔离相关邮件失败")?;

        Ok(())
    }

    /// 报告误报情报
    #[instrument(skip(self))]
    pub async fn report_false_positive(&self, log_id: Uuid, intelligence_id: Uuid, reason: String, provide_sample_email: bool) -> Result<()> {
        info!(
            "情报操作服务: 报告误报情报，日志ID: {}, 情报ID: {}, 原因: {}, 提供样本邮件: {}",
            log_id, intelligence_id, reason, provide_sample_email
        );

        // 检查情报是否存在
        self.check_intelligence_exists(intelligence_id).await?;

        // 如果有数据库连接，执行实际操作
        if let Some(client) = &self.db_client {
            info!("有数据库连接，将执行误报上报操作");
            self.execute_false_positive_report_db(client, log_id, intelligence_id, reason.clone(), provide_sample_email).await
                .context("执行误报上报数据库操作失败")?;
        } else {
            info!("无数据库连接，仅模拟误报上报操作");
            // 在内存模式下模拟操作成功
        }

        // 如果用户愿意提供样本邮件，记录该信息
        if provide_sample_email {
            self.record_sample_email_provision(log_id, intelligence_id).await
                .context("记录样本邮件提供意向失败")?;
        }

        Ok(())
    }

    /// 上报情报（原方法保留，但已不再使用）
    #[deprecated]
    #[instrument(skip(self))]
    pub async fn report_intelligence(&self, intelligence_id: Uuid, value: String, reason: Option<String>) -> Result<()> {
        info!(
            "情报操作服务: 上报情报，情报ID: {}, 情报值: {}, 原因: {:?}",
            intelligence_id, value, reason
        );

        // 检查情报是否存在
        self.check_intelligence_exists(intelligence_id).await?;

        // 如果有数据库连接，执行实际操作
        if let Some(client) = &self.db_client {
            info!("有数据库连接，将执行情报上报操作");
            self.execute_report_db(client, intelligence_id, value, reason.clone()).await
                .context("执行情报上报数据库操作失败")?;
        } else {
            info!("无数据库连接，仅模拟情报上报操作");
            // 在内存模式下模拟操作成功
        }

        Ok(())
    }

    // 辅助方法：检查情报是否存在
    async fn check_intelligence_exists(&self, intelligence_id: Uuid) -> Result<()> {
        // 在实际实现中，这里应该查询数据库确认情报是否存在
        // 当前简化实现，始终返回成功
        info!("检查情报是否存在: {}", intelligence_id);
        Ok(())
    }

    // 辅助方法：放行隔离区邮件
    async fn release_quarantined_emails(&self, intelligence_id: Uuid) -> Result<()> {
        info!("放行与情报相关的隔离邮件: {}", intelligence_id);
        // 在实际实现中，这里应该执行邮件放行操作
        Ok(())
    }

    // 辅助方法：隔离相关邮件
    async fn quarantine_related_emails(&self, intelligence_id: Uuid) -> Result<()> {
        info!("隔离与情报相关的邮件: {}", intelligence_id);
        // 在实际实现中，这里应该执行邮件隔离操作
        Ok(())
    }

    // 辅助方法：记录用户愿意提供样本邮件
    async fn record_sample_email_provision(&self, log_id: Uuid, intelligence_id: Uuid) -> Result<()> {
        info!("记录用户愿意提供样本邮件: 日志ID: {}, 情报ID: {}", log_id, intelligence_id);
        // 在实际实现中，这里应该记录用户意向
        Ok(())
    }

    // 数据库操作：执行白名单操作
    async fn execute_whitelist_db(&self, _client: &ClickHouseClient, intelligence_id: Uuid, value: String, _release_quarantine: bool) -> Result<()> {
        info!("执行白名单数据库操作: {}, 情报值: {}", intelligence_id, value);
        // 实际的数据库操作将在此处实现
        // 当前简化实现，始终返回成功
        Ok(())
    }

    // 数据库操作：执行黑名单操作
    async fn execute_blacklist_db(&self, _client: &ClickHouseClient, intelligence_id: Uuid, value: String) -> Result<()> {
        info!("执行黑名单数据库操作: {}, 情报值: {}", intelligence_id, value);
        // 实际的数据库操作将在此处实现
        // 当前简化实现，始终返回成功
        Ok(())
    }

    // 数据库操作：执行误报情报上报操作
    async fn execute_false_positive_report_db(&self, _client: &ClickHouseClient, log_id: Uuid, intelligence_id: Uuid, reason: String, provide_sample_email: bool) -> Result<()> {
        info!("执行误报上报数据库操作: 日志ID: {}, 情报ID: {}, 原因: {}, 提供样本: {}", log_id, intelligence_id, reason, provide_sample_email);
        // 实际的数据库操作将在此处实现
        // 当前简化实现，始终返回成功
        Ok(())
    }

    // 数据库操作：执行情报上报操作（保留旧方法，已不再使用）
    #[deprecated]
    async fn execute_report_db(&self, _client: &ClickHouseClient, intelligence_id: Uuid, value: String, _reason: Option<String>) -> Result<()> {
        info!("执行情报上报数据库操作: {}, 情报值: {}", intelligence_id, value);
        // 实际的数据库操作将在此处实现
        // 当前简化实现，始终返回成功
        Ok(())
    }
} 