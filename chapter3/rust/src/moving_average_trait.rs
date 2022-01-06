#[derive(Debug, Clone)]
// 型パラメータTを用いる
pub struct MovingAverage<T>
where
    // TはNumトレイトを満たす型、かつf64型にキャストできる型
    // 具体的にはu8, u32, u64, f32, f64など
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
            // Numトレイトを満たす型はzeroという関数を持つ
            // i32なら0, f64なら0.0を返すだろう
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
            // self.sumはT型、割り算するので出力型はf64とした
            true => Some(self.sum.as_() / self.period as f64),
            false => None,
        }
    }
}

fn calc_stream(average_length: usize) -> Vec<f64> {
    // i32型の数列
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
