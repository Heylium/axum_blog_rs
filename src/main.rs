use std::sync::Arc;

use axum::{Router,  extract::{Extension, extractor_middleware}};
use axum_blog::{handler::{backend, frontend }, config, AppState, middleware};
use deadpool_postgres::Runtime;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs_blog=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().expect("初始化配置失败");
    let pool = cfg.pg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).expect("创建数据库连接池失败");


    let frontend_routers = frontend::router();
    let backend_routers = backend::router().layer(extractor_middleware::<middleware::Auth>());
    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState { pool})));

    tracing::info!("服务已启动：{}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}