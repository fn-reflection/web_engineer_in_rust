from datetime import datetime
import math
import numpy as np
import pandas as pd
import numba
import psutil


def process_memory_usage_mb():
    return psutil.Process().memory_info().rss/1e6



def calc_batch_ndarray(calc_strategy, average_length) -> np.ndarray:
    before_read = datetime.utcnow()  # データ読み込み前の時刻記録
    nums = pd.read_csv("../data/time_series.csv")['value'].values  # データ一括読み込み
    after_read = datetime.utcnow()  # データ読み込み後の時刻記録
    moving_averages = calc_strategy(nums, average_length)  # 関数を用いて移動平均計算
    after_calc = datetime.utcnow()  # 移動平均計算後の時刻記録
    print(f"移動平均計算に使用した関数：{calc_strategy}")
    print(f"移動平均の長さ：{average_length}")
    print(f"移動平均の最後の要素：{moving_averages[-1]}")
    print(f"csvロードにかかった時間：{after_read - before_read }秒")
    print(f"移動平均計算にかかった時間：{after_calc - after_read}秒")
    print(f"配列のメモリ使用量(参考)：{moving_averages.nbytes/1e6}MB")
    print(f"プロセスメモリ使用量(参考)：{process_memory_usage_mb()}MB")
    return moving_averages


def moving_average_batch_numpy(nums: np.ndarray, average_length: int) -> np.ndarray:  # numpy arrayを使う、convolve APIを使う
    assert len(nums) - average_length + 1 > 0
    return np.convolve(nums, np.ones(average_length), 'valid') / average_length


def moving_average_batch_pandas(nums: np.ndarray, average_length: int) -> np.ndarray:  # pandasのrolling APIを使う
    assert len(nums) - average_length + 1 > 0
    return (pd.Series(nums).rolling(average_length).mean()).values[average_length-1:]


@numba.jit(nopython=True)
# numpy arrayを使う、素直に計算する、numbaを使う
def moving_average_batch_numpy_numba_naive(nums: np.ndarray, average_length: int) -> np.ndarray:
    assert len(nums) - average_length + 1 > 0
    N_i = nums.shape[0]
    res = np.empty_like(nums, dtype=np.float64)
    for i in range(average_length-1, N_i):
        res[i] = np.sum(nums[i-average_length+1:i+1]) / average_length
    return res[average_length-1:]


@numba.jit(nopython=True)
# numpy arrayを使う、オンラインアルゴリズムを使用、numbaを使う
def moving_average_batch_numpy_numba_online(nums: np.ndarray, average_length: int) -> np.ndarray:
    assert len(nums) - average_length + 1 > 0
    N_i = nums.shape[0]
    res = np.empty_like(nums, dtype=np.float64)
    res[average_length - 1] = np.sum(nums[:average_length])
    for i in range(average_length, N_i):
        res[i] = nums[i] - nums[i - average_length] + res[i - 1]
    for i in range(average_length-1, N_i):
        res[i] = res[i] / average_length
    return res[average_length-1:]


if __name__ == "__main__":
    ma1 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy, average_length=7)
    ma2 = calc_batch_ndarray(calc_strategy=moving_average_batch_pandas, average_length=7)
    assert math.isclose(np.sum(ma1-ma2), 0)  # ma1とma2はほぼ同じであることを確認
    ma3 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy_numba_naive, average_length=7)
    assert math.isclose(np.sum(ma1-ma3), 0)  # ma1とma3はほぼ同じであることを確認
    ma4 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy_numba_online, average_length=7)
    assert math.isclose(np.sum(ma1-ma4), 0)  # ma1とma4はほぼ同じであることを確認
    ma5 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy, average_length=5000)
    ma6 = calc_batch_ndarray(calc_strategy=moving_average_batch_pandas, average_length=5000)
    assert math.isclose(np.sum(ma5-ma6), 0)  # ma5とma6はほぼ同じであることを確認
    ma7 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy_numba_naive, average_length=5000)
    assert math.isclose(np.sum(ma5-ma6), 0)  # ma5とma7はほぼ同じであることを確認
    ma8 = calc_batch_ndarray(calc_strategy=moving_average_batch_numpy_numba_online, average_length=5000)
    assert math.isclose(np.sum(ma5-ma6), 0)  # ma5とma8はほぼ同じであることを確認

# poetry run python run_calc_batch_numpy.py
# 移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f7e4ab09b80>
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.551404秒
# 移動平均計算にかかった時間：0:00:00.029370秒
# 移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f7e4ab09c10>
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.544876秒
# 移動平均計算にかかった時間：0:00:00.124585秒
# 移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f7e4ab09d30>)
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.554755秒
# 移動平均計算にかかった時間：0:00:00.275042秒
# 移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f7e4ab09f70>)
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.545795秒
# 移動平均計算にかかった時間：0:00:00.198909秒
# 移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f7e4ab09b80>
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.537312秒
# 移動平均計算にかかった時間：0:00:05.997577秒
# 移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f7e4ab09c10>
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.548041秒
# 移動平均計算にかかった時間：0:00:00.123680秒
# 移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f7e4ab09d30>)
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.548829秒
# 移動平均計算にかかった時間：0:00:32.939087秒
# 移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f7e4ab09f70>)
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.513963秒
# 移動平均計算にかかった時間：0:00:00.049948秒
