/// 应用程序入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载.env文件
    match dotenv::dotenv() {
        Ok(path) => println!("已加载.env文件: {}", path.display()),
        Err(e) => println!("加载.env文件失败: {}", e),
    }
    
    // 运行服务器
    analysis_api::run_server().await
}
