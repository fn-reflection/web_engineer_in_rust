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

fn calc_stream(average_length: usize) -> Vec<f64> {
    let input_data = 1..=10;
    let mut ma = MovingAverage::new(average_length);
    let moving_averages = input_data
        // .map(|n| n as f64) // このf64への変換を入れれば型があうが・・・
        .filter_map(|new_val| ma.latest(new_val))
        .collect::<Vec<_>>();
    moving_averages
}

pub fn main() -> () {
    let ma = calc_stream(2);
    println!("{:?}", ma);
}
