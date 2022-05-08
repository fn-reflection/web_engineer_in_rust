use web_engineer_in_rust::{create_pool, create_tokio_runtime, IrisMeasurement, DB_STRING_PRODUCTION};

fn main() -> anyhow::Result<()> {
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    // 本番データベースに接続するクライアントプールを作成
    let pool = create_pool(DB_STRING_PRODUCTION).await?;
    // 本番データベースにiris_measurementsテーブルを作成
    let query_result = IrisMeasurement::create_table(&pool).await?;
    println!("{:?}", query_result);
    Ok(())
}

