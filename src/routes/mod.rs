// 导出所有路由模块
mod intelligence;
mod email;
mod timeline;
mod statistics;
mod trend;
mod system;

// 重新导出所有处理函数，使其可以通过routes模块访问
pub use intelligence::*;
pub use email::*;
pub use timeline::*;
pub use statistics::*;
pub use trend::*;
pub use system::*;

// 定义路由构建函数
pub mod router; 