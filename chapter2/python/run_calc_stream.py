from datetime import datetime
import csv
import sys
import collections
import numpy as np
import pandas as pd
import numba
import psutil


def process_memory_usage_mb():
    return psutil.Process().memory_info().rss/1e6


def read_csv(relative_path):
    res = []
    with open(relative_path) as f:
        reader = csv.reader(f)
        next(reader) # ヘッダーをskipする
        for row in reader:
            res.append(float(row[0])) # フロート型に変換して読み込む
    return res

class MovingAveragePython(object):
    def __init__(self, period):
        self.sum = 0
        self.period = period
        self.deque = collections.deque()

    def latest(self, new_val):
        self.deque.append(new_val)
        old_val = self.deque.popleft() if len(self.deque) > self.period else 0.0
        self.sum += new_val - old_val
        return self.sum / self.period if len(self.deque) == self.period else None


@numba.experimental.jitclass([
    ('sum', numba.float64), ('buffer', numba.float64[:]), ('head', numba.int64), ('tail', numba.int64)
])
class MovingAverageNumba(object):
    def __init__(self, period):
        self.sum = 0
        # collections.dequeを使いたくても対応していない
        # jitclassを用いたclassを使いたくても対応していない
        # self.deque = collections.deque(period)
        self.buffer = np.zeros((period,), dtype=np.float64)
        self.head = 0
        self.tail = 0

    def latest(self, new_val):
        period = len(self.buffer)
        self.tail = (self.tail + 1) % period
        old_val = self.buffer[self.head] if self.head == self.tail else 0
        self.sum += new_val - old_val
        self.buffer[self.tail] = new_val
        if self.head == self.tail:
            self.head = (self.head + 1) % period
            return self.sum / period
        else:
            return np.nan


@numba.jit(nopython=True)
# numpy arrayを使う、素直に計算する、numbaのjitclassを使う
def moving_average_batch_numpy_numba_jitclass(nums: np.ndarray, average_length: int) -> np.ndarray:
    assert len(nums) - average_length + 1 > 0
    N_i = nums.shape[0]
    res = np.zeros_like(nums, dtype=np.float64)
    ma = MovingAverageNumba(average_length)
    for i in range(average_length-1):
        res[i] = ma.latest(nums[i])
    for i in range(average_length-1, N_i):
        res[i] = ma.latest(nums[i])
    return res[average_length-1:]


def calc_batch(average_length) -> np.ndarray:
    before_read = datetime.utcnow()  # データ読み込み前の時刻記録
    nums = pd.read_csv("../data/time_series.csv")['value'].values  # データ一括読み込み
    after_read = datetime.utcnow()  # データ読み込み後の時刻記録
    moving_averages = moving_average_batch_numpy_numba_jitclass(nums, average_length)  # jitclassを用いて移動平均計算
    after_calc = datetime.utcnow()  # 移動平均計算後の時刻記録
    print(f"移動平均の長さ：{average_length}")
    print(f"移動平均の最後の要素：{moving_averages[-1]}")
    print(f"csvロードにかかった時間：{after_read - before_read }秒")
    print(f"移動平均計算にかかった時間：{after_calc - after_read}秒")
    return moving_averages


def calc_stream(constructor, average_length) -> np.ndarray:
    before_read = datetime.utcnow()  # データ読み込み前の時刻記録
    moving_averages = []
    ma = constructor(average_length)
    with open("../data/time_series.csv") as f:
        reader = csv.reader(f)
        next(reader) # ヘッダーをskipする
        for row in reader:
            num = float(row[0])
            moving_averages.append(ma.latest(num))
    after_calc = datetime.utcnow()  # 移動平均計算後の時刻記録
    print(f"移動平均の長さ：{average_length}")
    print(f"移動平均の最後の要素：{moving_averages[-1]}")
    print(f"計算にかかった時間：{after_calc - before_read}秒")
    print(f"リストのメモリ使用量(参考)：{sys.getsizeof(moving_averages)/1e6}MB")
    print(f"プロセスメモリ使用量(参考)：{process_memory_usage_mb()}MB")
    return moving_averages


if __name__ == "__main__":
    ma1 = calc_stream(constructor=MovingAveragePython, average_length=7)
    ma2 = calc_stream(constructor=MovingAverageNumba, average_length=7)
    ma3 = calc_stream(constructor=MovingAveragePython, average_length=5000)
    ma4 = calc_stream(constructor=MovingAverageNumba, average_length=5000)
    ma5 = calc_batch(average_length=7)
    ma6 = calc_batch(average_length=5000)

    print(ma1[:10], ma1[-10:])
    print(ma2[:10], ma2[-10:])
    print(ma3[:10], ma3[-10:])
    print(ma4[:10], ma4[-10:])
    print(ma5[:10], ma5[-10:])
    print(ma6[:10], ma6[-10:])
