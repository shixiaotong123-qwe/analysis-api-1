use anyhow::Context;
use clickhouse::{Client, Row};
use tracing::{debug, info, error};
use serde::{de::DeserializeOwned, Serialize};
use crate::db::{DbConfig, /* DbError, */ DbResult};

/// ClickHouse客户端
#[derive(Clone)]
pub struct ClickHouseClient {
    /// 内部客户端
    client: Client,
    /// 数据库名称
    database: String,
}

impl ClickHouseClient {
    /// 创建一个新的ClickHouse客户端
    pub async fn new(config: DbConfig) -> DbResult<Self> {
        // 构建数据库URL和客户端
        let mut client = Client::default()
            .with_url(&config.url);
        
        // 如果提供了认证信息，单独设置
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            client = client
                .with_user(username)
                .with_password(password);
        }
        
        // 设置数据库
        client = client.with_database(&config.database);
        
        info!("正在连接ClickHouse数据库: {}，数据库: {}", 
              config.url,
              config.database);
        
        // 测试连接
        let client = client;
        ping(&client).await?;
        
        Ok(Self {
            client,
            database: config.database,
        })
    }
    
    /// 获取内部客户端引用
    pub fn inner(&self) -> &Client {
        &self.client
    }
    
    /// 获取数据库名称
    pub fn database(&self) -> &str {
        &self.database
    }
    
    /// 执行查询并返回行列表
    pub async fn query<R>(&self, query: &str) -> DbResult<Vec<R>> 
    where
        R: Row + DeserializeOwned + Send + 'static,
    {
        debug!("执行ClickHouse查询: {}", query);
        let result = self.client.query(query).fetch_all().await
            .context("执行ClickHouse查询失败")?;
        Ok(result)
    }
    
    /// 执行插入操作
    pub async fn insert<R>(&self, table: &str, data: Vec<R>) -> DbResult<()> 
    where
        R: Row + Serialize + Send + 'static,
    {
        debug!("向表 {} 插入数据，行数: {}", table, data.len());
        let mut insert = self.client.insert(table)?;
        for row in data {
            insert.write(&row).await?;
        }
        insert.end().await.context(format!("向表 {} 插入数据失败", table))?;
        Ok(())
    }
    
    /// 执行不返回结果的查询（如CREATE TABLE等）
    pub async fn exec(&self, query: &str) -> DbResult<()> {
        debug!("执行ClickHouse命令: {}", query);
        self.client.query(query).execute().await
            .context("执行ClickHouse命令失败")?;
        Ok(())
    }
}

/// 测试数据库连接
async fn ping(client: &Client) -> DbResult<()> {
    info!("尝试连接到ClickHouse数据库，正在执行 'SELECT 1' 测试查询...");
    
    let ping_result = client.query("SELECT 1").execute().await;
    match ping_result {
        Ok(_) => {
            info!("成功连接到ClickHouse数据库");
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("测试ClickHouse连接失败: {}\n请检查: 1) URL是否正确 2) 用户名和密码是否正确 3) 数据库名称是否存在 4) 网络连接是否正常", e);
            error!("{}", error_msg);
            Err(anyhow::anyhow!(error_msg))
        }
    }
} 