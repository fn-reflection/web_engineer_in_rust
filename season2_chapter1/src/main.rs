use web_engineer_in_rust::{
    create_pool, create_tokio_runtime, read_csv, IrisMeasurement, DB_STRING_PRODUCTION,
};

fn main() -> anyhow::Result<()> {
    let tokio_rt = create_tokio_runtime();
    tokio_rt.block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let measurements = read_csv("data/iris.csv")?;
    let pool = create_pool(DB_STRING_PRODUCTION).await?;
    for m in measurements {
        m.insert(&pool).await?;
    }
    let rows = IrisMeasurement::find_by_class(&pool, "Iris-versicolor").await?;
    println!("{:?}", rows);
    Ok(())
}
