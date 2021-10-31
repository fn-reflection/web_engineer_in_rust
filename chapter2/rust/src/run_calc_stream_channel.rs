fn get_csv_path(relative_path: &str) -> std::path::PathBuf {
    let project_path = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_path)
        .parent()
        .unwrap()
        .join(relative_path)
}

fn read_csv(relative_path: &str) -> anyhow::Result<std::sync::mpsc::Receiver<f64>> {
    let (tx, rx) = std::sync::mpsc::channel::<f64>();
    let csv_path = get_csv_path(relative_path);
    let mut csv_reader = csv::Reader::from_path(csv_path)?;
    let _ = std::thread::spawn(move || {
        csv_reader
            .deserialize::<f64>()
            .filter_map(|row_result| row_result.ok())
            .for_each(|row| tx.send(row).unwrap())
    });
    Ok(rx)
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

fn calc_stream_channel(average_length: usize) -> anyhow::Result<Vec<f64>> {
    let before_read = chrono::Utc::now();
    let mut ma = MovingAverage::new(average_length);
    let moving_averages = read_csv("data/time_series.csv")?
        .into_iter()
        .filter_map(|new_val| ma.latest(new_val))
        .collect::<Vec<_>>();
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
    let _ma2 = calc_stream_channel(7)?;
    let _ma4 = calc_stream_channel(5000)?;
    Ok(())
}
