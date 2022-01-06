#[derive(Debug, Clone)]
pub struct MovingAverage<T>
where
    T: num_traits::Num + num_traits::cast::AsPrimitive<f64>,
{
    period: usize,
    sum: T,
    deque: std::collections::VecDeque<T>,
}

impl<T> MovingAverage<T>
where
    T: num_traits::Num + num_traits::cast::AsPrimitive<f64>,
{
    pub fn new(period: usize) -> Self {
        Self {
            period,
            sum: T::zero(),
            deque: std::collections::VecDeque::new(),
        }
    }

    pub fn latest(&mut self, new_val: T) -> Option<f64> {
        self.deque.push_back(new_val);
        let old_val = match self.deque.len() > self.period {
            true => self.deque.pop_front().unwrap(),
            false => T::zero(),
        };
        self.sum = self.sum + new_val - old_val;
        match self.deque.len() == self.period {
            true => Some(self.sum.as_() / self.period as f64),
            false => None,
        }
    }
}

fn calc_stream(average_length: usize) -> Vec<f64> {
    let input_data = 1..=10;
    let mut ma = MovingAverage::new(average_length);
    let moving_averages = input_data
        .filter_map(|new_val| ma.latest(new_val))
        .collect::<Vec<_>>();
    moving_averages
}

pub fn main() -> () {
    let ma = calc_stream(2);
    println!("{:?}", ma);
}
