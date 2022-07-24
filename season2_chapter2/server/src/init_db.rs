// src/init_db.rs
use ruitter::models::{create_pool, create_tokio_runtime, setup_tables, DB_STRING_PRODUCTION};

fn main() -> anyhow::Result<()> {
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    // 本番データベースに接続するクライアントプールを作成
    let pool = create_pool(DB_STRING_PRODUCTION).await?;
    let session_store = async_sqlx_session::MySqlSessionStore::new(DB_STRING_PRODUCTION).await?;
    session_store.migrate().await?;

    // 本番データベースにiris_measurementsテーブルを作成
    setup_tables(&pool).await;
    Ok(())
}
