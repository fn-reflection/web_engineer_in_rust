const SIMD_LANES: usize = 16;

// window.iter().sum::<f64>の代わりに使うと、大きいNに対して有効
pub fn simd_sum(values: &[f64]) -> f64 {
    let chunks = values.chunks_exact(SIMD_LANES);
    let remainder = chunks.remainder();

    let sum = chunks.fold([0.0; SIMD_LANES], |mut acc, chunk| {
        let chunk: [f64; SIMD_LANES] = chunk.try_into().unwrap();
        for i in 0..SIMD_LANES {
            acc[i] += chunk[i];
        }
        acc
    });

    let remainder: f64 = remainder.iter().copied().sum();

    let mut reduced = 0.0;
    for i in 0..SIMD_LANES {
        reduced += sum[i];
    }
    reduced + remainder
}

fn moving_average_batch_naive(nums: &[f64], average_length: usize) -> anyhow::Result<Vec<f64>> {
    let size = nums.len() as i64 - average_length as i64 + 1;
    if size <= 0 {
        return Err(anyhow::anyhow!(
            "average length must be less than nums array length"
        ));
    }
    let averages = nums
        .windows(average_length)
        .map(|window| window.iter().sum::<f64>() / (window.len() as f64))
        .collect::<Vec<_>>();
    Ok(averages)
}

fn moving_average_batch_online(nums: &[f64], average_length: usize) -> anyhow::Result<Vec<f64>> {
    let size = nums.len() as i64 - average_length as i64 + 1;
    if size <= 0 {
        return Err(anyhow::anyhow!(
            "average length must be less than nums array length"
        ));
    }
    let mut res = Vec::with_capacity(nums.len());
    res.push(nums[0..average_length].iter().sum::<f64>());
    for i in average_length..nums.len() {
        res.push(nums[i] as f64 - nums[i - average_length] as f64 + res[i - average_length])
    }
    for i in 0..(nums.len() - average_length + 1) {
        res[i] /= average_length as f64;
    }
    Ok(res)
}

fn get_csv_path(relative_path: &str) -> std::path::PathBuf {
    let project_path = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_path)
        .parent()
        .unwrap()
        .join(relative_path)
}

fn read_csv(relative_path: &str) -> anyhow::Result<Vec<f64>> {
    let csv_path = get_csv_path(relative_path);
    let mut csv_reader = csv::Reader::from_path(csv_path)?;
    let nums = csv_reader
        .deserialize::<f64>()
        .filter_map(|row_result| row_result.ok())
        .collect::<Vec<_>>();
    Ok(nums)
}

fn calc_batch<F: FnOnce(&[f64], usize) -> anyhow::Result<Vec<f64>>>(
    strategy: F,
    average_length: usize,
) -> anyhow::Result<Vec<f64>> {
    let before_read = chrono::Utc::now();
    let nums = read_csv("data/time_series.csv")?;
    let after_read = chrono::Utc::now();
    let moving_averages = strategy(&nums, average_length)?;
    let after_calc = chrono::Utc::now();
    println!(
        "移動平均計算に使用した関数：{:?}",
        std::any::type_name::<F>()
    );
    println!("移動平均の長さ：{}", average_length);
    println!(
        "移動平均の最後の要素：{:?}",
        moving_averages[moving_averages.len() - 1]
    );
    let load_time = after_read - before_read;
    let calc_time = after_calc - after_read;
    println!(
        "csvロードにかかった時間：{:?}秒",
        load_time.num_nanoseconds().unwrap() as f64 / 1e9
    );
    println!(
        "移動平均計算にかかった時間：{:?}秒",
        calc_time.num_nanoseconds().unwrap() as f64 / 1e9
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
    let ma1 = calc_batch(moving_average_batch_naive, 7)?;
    let ma2 = calc_batch(moving_average_batch_online, 7)?;
    assert_eq!(ma1, ma2);
    let _ma3 = calc_batch(moving_average_batch_naive, 5000)?;
    let _ma4 = calc_batch(moving_average_batch_online, 5000)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_data_ok() {
        let actual = read_csv("data/time_series_test.csv").unwrap();
        let expect = vec![101.0, 102.0, 103.0];
        assert_eq!(actual, expect);
    }

    #[test]
    fn moving_average_naive_ok() {
        let actual = moving_average_batch_naive(&vec![1.0, 2.0, 3.0, 4.0, 5.0], 2).unwrap();
        let expect = vec![1.5, 2.5, 3.5, 4.5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn moving_average_batch_online_ok() {
        let actual = moving_average_batch_online(&vec![1.0, 2.0, 3.0, 4.0, 5.0], 2).unwrap();
        let expect = vec![1.5, 2.5, 3.5, 4.5];
        assert_eq!(actual, expect);
    }

    #[test]
    fn moving_average_naive_err() {
        let actual = moving_average_batch_naive(&vec![1.0, 2.0, 3.0], 4);
        assert!(actual.is_err());
    }

    #[test]
    fn moving_average_batch_online_err() {
        let actual = moving_average_batch_online(&vec![1.0, 2.0, 3.0], 2).unwrap();
        let expect = vec![1.5, 2.5];
        assert_eq!(actual, expect);
    }
}

/*
cargo run --release --bin run_calc
=>
移動平均計算に使用した関数："run_calc::moving_average_batch_naive"
移動平均の長さ：7
移動平均の最後の要素：1428573.5714285714
csvロードにかかった時間：0.575756919秒
移動平均計算にかかった時間：0.043552966秒
移動平均計算に使用した関数："run_calc::moving_average_batch_online"
移動平均の長さ：7
移動平均の最後の要素：1428573.5714285714
csvロードにかかった時間：0.532375165秒
移動平均計算にかかった時間：0.080982676秒
移動平均計算に使用した関数："run_calc::moving_average_batch_naive"
移動平均の長さ：5000
移動平均の最後の要素：1428216.9284
csvロードにかかった時間：0.538983387秒
移動平均計算にかかった時間：32.649335643秒
移動平均計算に使用した関数："run_calc::moving_average_batch_online"
移動平均の長さ：5000
移動平均の最後の要素：1428216.9284
csvロードにかかった時間：0.507449123秒
移動平均計算にかかった時間：0.075278821秒
 */
