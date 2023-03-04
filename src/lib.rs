pub mod error;
pub mod db;
pub mod handler;
mod view;
pub mod config;
mod form;
mod model;
mod md;

pub type Result<T> = std::result::Result<T, error::AppError>;


pub struct AppState {
    /// 数据库连接
    pub pool: deadpool_postgres::Pool,
}