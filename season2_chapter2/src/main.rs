// src/main.rs
use web_engineer_in_rust::endpoints::run_server;
use web_engineer_in_rust::models::{create_pool, create_tokio_runtime, DB_STRING_PRODUCTION};

fn main() -> anyhow::Result<()> {
    // 非同期ランタイムを生成
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let pool = create_pool(DB_STRING_PRODUCTION).await?;
    let session_store = async_sqlx_session::MySqlSessionStore::new(DB_STRING_PRODUCTION).await?;
    run_server(pool, session_store).await
}
