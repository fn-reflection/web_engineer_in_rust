use polars::prelude::*;
use polars_core::datatypes::DataType;
use polars_lazy::prelude::*;
//use polars_core::chunked_array::ops::rolling_window::RollingOptions;
//use polars_lazy::prelude::{col, LazyCsvReader};

fn get_csv_path(relative_path: &str) -> std::path::PathBuf {
    let project_path = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_path)
        .parent()
        .unwrap()
        .join(relative_path)
}

#[derive(Debug, Clone)]
pub struct MovingAverage {
    period: usize,
    sum: f64,
    deque: std::collections::VecDeque<f64>,
}

impl MovingAverage {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            sum: 0.0,
            deque: std::collections::VecDeque::new(),
        }
    }

    pub fn latest(&mut self, new_val: f64) -> Option<f64> {
        self.deque.push_back(new_val);
        let old_val = match self.deque.len() > self.period {
            true => self.deque.pop_front().unwrap(),
            false => 0.0,
        };
        self.sum += new_val - old_val;
        match self.deque.len() == self.period {
            true => Some(self.sum / self.period as f64),
            false => None,
        }
    }
}
fn calc_stream(average_length: usize) -> anyhow::Result<polars::frame::DataFrame> {
    let before_read = chrono::Utc::now();
    let csv_path = get_csv_path("data/time_series.csv");
    let frame = LazyCsvReader::new(csv_path.into_os_string().into_string().unwrap())
        .finish()
        .unwrap();
    let ma = std::sync::Arc::new(std::sync::Mutex::new(MovingAverage::new(average_length)));
    let moving_averages = frame
        .with_column(
            col("value")
                .map(
                    move |s| {
                        let x = s
                            .f64()?
                            .apply(|num| ma.lock().unwrap().latest(num).unwrap_or(std::f64::NAN));
                        Ok(x.into())
                    },
                    GetOutput::from_type(DataType::Float64),
                )
                .alias("ma"),
        )
        .select(&[col("ma")])
        .collect()?;
    let after_calc = chrono::Utc::now();
    println!("移動平均の長さ：{}", average_length);
    println!("移動平均の最後の要素：{:?}", moving_averages);
    let total = after_calc - before_read;
    println!(
        "計算にかかった時間：{:?}秒",
        total.num_nanoseconds().unwrap() as f64 / 1e9
    );
    println!(
        "Vecの使用メモリ量(参考)：{:?}MB",
        std::mem::size_of_val(&moving_averages) as f64 / 1e6
    );
    println!(
        "プロセスの使用メモリ量(参考)：{:?}MB",
        psutil::process::Process::new(std::process::id())
            .unwrap()
            .memory_info()
            .unwrap()
            .rss() as f64
            / 1e6
    );
    Ok(moving_averages)
}

fn main() -> anyhow::Result<()> {
    let ma4 = calc_stream(5000)?;
    Ok(())
}
