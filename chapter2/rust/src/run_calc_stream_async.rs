use tokio_stream::StreamExt as _;

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

async fn calc_stream_channel(average_length: usize) -> anyhow::Result<Vec<f64>> {
    let before_read = chrono::Utc::now();
    let csv_path = get_csv_path("data/time_series.csv");
    let mut csv_reader =
        csv_async::AsyncDeserializer::from_reader(tokio::fs::File::open(csv_path).await?);
    let mut ma = MovingAverage::new(average_length);
    let mut stream_nums = csv_reader.deserialize::<f64>();
    let mut moving_averages = vec![];
    while let Some(num) = stream_nums.next().await {
        let new_val = ma.latest(num?);
        if new_val.is_some() {
            moving_averages.push(new_val.unwrap());
        }
    }
    let after_calc = chrono::Utc::now();
    println!("移動平均の長さ：{}", average_length);
    println!(
        "移動平均の最後の要素：{:?}",
        moving_averages[moving_averages.len() - 1]
    );
    let total = after_calc - before_read;
    println!(
        "計算にかかった時間：{:?}秒",
        total.num_nanoseconds().unwrap() as f64 / 1e9
    );
    println!(
        "Vecの使用メモリ量(参考)：{:?}MB",
        std::mem::size_of_val(&*moving_averages) as f64 / 1e6
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
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let _ma1 = calc_stream_channel(7).await;
        let _ma2 = calc_stream_channel(5000).await;
    });
    Ok(())
}
