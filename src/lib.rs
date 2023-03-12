pub mod error;
pub mod db;
pub mod handler;
pub mod view;
pub mod config;
pub mod form;
pub mod model;
mod md;
mod cookie;
pub mod middleware;
mod password;

pub type Result<T> = std::result::Result<T, error::AppError>;


pub struct AppState {
    /// 数据库连接
    pub pool: deadpool_postgres::Pool,
}