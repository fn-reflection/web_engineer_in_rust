// src/main.rs
use std::sync::Arc;
use ruitter::endpoints::run_server;
use ruitter::models::{create_pool, create_tokio_runtime, DB_STRING_PRODUCTION};

fn main() -> anyhow::Result<()> {
    // 非同期ランタイムを生成
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let arc_pool = Arc::new(create_pool(DB_STRING_PRODUCTION).await?);
    let session_store = async_sqlx_session::MySqlSessionStore::new(DB_STRING_PRODUCTION).await?;
    run_server(arc_pool, session_store).await
}
