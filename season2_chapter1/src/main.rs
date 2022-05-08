use web_engineer_in_rust::{
    create_pool, create_tokio_runtime, read_csv, IrisMeasurement, DB_STRING_PRODUCTION,
};

fn main() -> anyhow::Result<()> {
    // 非同期ランタイムを生成
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    // csvからデータセットをメモリにロード
    let measurements = read_csv("data/iris.csv")?;
    let pool = create_pool(DB_STRING_PRODUCTION).await?;
    // 一件ずつデータベースにINSERT
    for m in measurements {
        m.insert(&pool).await?;
    }
    // Iris-versicolorのデータを取得する
    let rows = IrisMeasurement::find_by_class(&pool, "Iris-versicolor").await?;
    println!("{:?}", rows);
    Ok(())
}
