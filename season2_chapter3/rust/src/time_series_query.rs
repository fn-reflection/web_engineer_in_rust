use polars::prelude::{col, DataFrame, Duration, LazyCsvReader, LazyFrame, RollingOptions};

// プロジェクトディレクトリからの相対パスを絶対パスにするユーティリティ
fn get_csv_path(relative_path: &str) -> std::path::PathBuf {
    let project_path = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_path)
        .parent()
        .unwrap()
        .join(relative_path)
}

// polarsで移動平均を計算する
fn by_polars(df: LazyFrame, window_size: Duration) -> anyhow::Result<DataFrame> {
    let rolling_options = RollingOptions {
        window_size,
        ..RollingOptions::default()
    };
    let features = df
        .select([col("value").rolling_mean(rolling_options)])
        .collect()?;
    Ok(features)
}

fn main() -> anyhow::Result<()> {
    let csv_path = get_csv_path("data/time_series.csv");
    // csvを遅延読み込みする
    let df = LazyCsvReader::new(csv_path).has_header(true).finish()?;
    // polarsでの特徴量計算(遅延評価する)
    let features = by_polars(df, Duration::new(50))?;
    println!("{:?}", features);
    Ok(())
}
