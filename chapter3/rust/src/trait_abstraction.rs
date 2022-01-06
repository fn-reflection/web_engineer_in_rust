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

fn calc_stream(average_length: usize) -> anyhow::Result<Vec<f64>> {
    let input_data = [1,2,3,4,5,6,7,8,9,10];
    let mut ma = MovingAverage::new(average_length);
    let moving_averages = input_data
        .filter_map(|new_val| ma.latest(new_val))
        .collect::<Vec<_>>();
    print(moving_averages);
    Ok(moving_averages)
}
