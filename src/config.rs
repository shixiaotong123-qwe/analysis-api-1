use std::net::{IpAddr, SocketAddr};
use crate::db::DbConfig;
use std::env;
use tracing::{info, debug};

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 服务器监听地址
    pub server_addr: SocketAddr,
    /// Jaeger地址
    pub jaeger_endpoint: String,
    /// 数据库配置
    pub db_config: DbConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 6000),
            jaeger_endpoint: "http://localhost:4317".to_string(),
            db_config: DbConfig::default(),
        }
    }
}

/// 从环境变量获取配置值，如果环境变量不存在则使用默认值
fn get_env_or_default<T: std::str::FromStr>(key: &str, default: T) -> T {
    match env::var(key) {
        Ok(val) => {
            debug!("从环境变量加载 {}: {}", key, val);
            val.parse::<T>().unwrap_or_else(|_| {
                debug!("无法解析环境变量 {}: {}，使用默认值", key, val);
                default
            })
        },
        Err(_) => {
            debug!("环境变量 {} 不存在，使用默认值", key);
            default
        },
    }
}

/// 从环境变量获取字符串值，如果环境变量不存在则使用默认值
fn get_env_string_or_default(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => {
            debug!("从环境变量加载 {}: {}", key, val);
            val
        },
        Err(_) => {
            debug!("环境变量 {} 不存在，使用默认值: {}", key, default);
            default.to_string()
        },
    }
}

/// 从环境变量获取可选字符串值
fn get_env_optional_string(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) if !val.is_empty() => {
            debug!("从环境变量加载 {}: {}", key, if key.contains("PASSWORD") { "***" } else { &val });
            Some(val)
        },
        _ => {
            debug!("环境变量 {} 不存在或为空", key);
            None
        },
    }
}

/// 获取应用配置
pub fn get_config() -> AppConfig {
    info!("加载应用配置...");
    
    // 服务器配置
    let server_ip_str = get_env_string_or_default("SERVER_IP", "127.0.0.1");
    let server_ip: IpAddr = server_ip_str.parse().unwrap_or_else(|_| {
        debug!("无法解析服务器IP: {}，使用默认值", server_ip_str);
        IpAddr::from([127, 0, 0, 1])
    });
    let server_port: u16 = get_env_or_default("SERVER_PORT", 6000);
    let server_addr = SocketAddr::new(server_ip, server_port);
    
    // Jaeger配置
    let jaeger_endpoint = get_env_string_or_default("JAEGER_ENDPOINT", "http://localhost:4317");
    
    // 数据库配置
    let db_url = get_env_string_or_default("DB_URL", "http://localhost:8123");
    let db_username = get_env_optional_string("DB_USERNAME");
    let db_password = get_env_optional_string("DB_PASSWORD");
    let db_name = get_env_string_or_default("DB_NAME", "default");
    
    let db_config = DbConfig {
        url: db_url,
        username: db_username,
        password: db_password,
        database: db_name.clone(),
    };
    
    info!("配置加载完成: 服务器地址={}, 数据库={}", server_addr, db_name);
    
    AppConfig {
        server_addr,
        jaeger_endpoint,
        db_config,
    }
} 