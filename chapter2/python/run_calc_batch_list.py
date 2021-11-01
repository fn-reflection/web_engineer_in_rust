import csv
import math
import sys
import psutil
from datetime import datetime
from typing import List
import pandas as pd


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

def calc_batch_list(calc_strategy, average_length) -> List:
    """
    csvからpythonのlistを読み込み、バッチ的に移動平均を計算させる
    """
    before_read = datetime.utcnow()  # データ読み込み前の時刻記録
    nums = read_csv("../data/time_series.csv") # データ一括読み込み
    after_read = datetime.utcnow()  # データ読み込み後の時刻記録
    moving_averages = calc_strategy(nums, average_length)  # 関数を用いて移動平均計算
    after_calc = datetime.utcnow()  # 移動平均計算後の時刻記録
    print(f"移動平均計算に使用した関数：{calc_strategy}")
    print(f"移動平均の長さ：{average_length}")
    print(f"移動平均の最後の要素：{moving_averages[-1]}")
    print(f"csvロードにかかった時間：{after_read - before_read }秒")
    print(f"移動平均計算にかかった時間：{after_calc - after_read}秒")
    print(f"リストのメモリ使用量(参考)：{sys.getsizeof(moving_averages)/1e6}MB")
    print(f"プロセスメモリ使用量(参考)：{process_memory_usage_mb()}MB")
    return moving_averages


def moving_average_batch_python(nums: List, average_length: int) -> List:
    """
    pythonのlistを使う、移動平均を素直に計算する
    """
    assert len(nums) - average_length + 1 > 0  # 移動平均が計算できない場合は例外送出する
    res = [sum(nums[i-average_length+1:i+1]) / average_length for i in range(average_length-1, len(nums))]
    return res


def moving_average_batch_python_online(nums: List, average_length: int) -> List:
    """
    python listを使う、移動平均をオンラインアルゴリズムで計算する
    """
    assert len(nums) - average_length + 1 > 0
    N_i = len(nums)
    res = [0]*N_i
    res[average_length - 1] = sum(nums[:average_length])
    for i in range(average_length, N_i):
        res[i] = nums[i] - nums[i - average_length] + res[i - 1]
    for i in range(average_length-1, N_i):
        res[i] = res[i] / average_length
    return res[average_length-1:]


if __name__ == "__main__":
    ma1 = calc_batch_list(calc_strategy=moving_average_batch_python, average_length=7)
    ma2 = calc_batch_list(calc_strategy=moving_average_batch_python_online, average_length=7)
    assert math.isclose(sum([ma1[i]-ma2[i] for i in range(len(ma1))]), 0)  # ma1とma2はほぼ同じであることを確認
    ma3 = calc_batch_list(calc_strategy=moving_average_batch_python, average_length=5000)
    ma4 = calc_batch_list(calc_strategy=moving_average_batch_python_online, average_length=5000)
    assert math.isclose(sum([ma3[i]-ma4[i] for i in range(len(ma3))]), 0)  # ma3とma4はほぼ同じであることを確認

# poetry run python run_calc_batch_list.py
# =>
# 移動平均計算に使用した関数：<function moving_average_batch_python at 0x7f656a23f9d0>
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.729763秒
# 移動平均計算にかかった時間：0:00:06.304983秒
# 移動平均計算に使用した関数：<function moving_average_batch_python_online at 0x7f656a23fa60>
# 移動平均の長さ：7
# 移動平均の最後の要素：1428573.5714285714
# csvロードにかかった時間：0:00:00.726929秒
# 移動平均計算にかかった時間：0:00:03.034528秒
# 移動平均計算に使用した関数：<function moving_average_batch_python at 0x7f656a23f9d0>
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.726875秒
# 移動平均計算にかかった時間：0:31:54.412072秒
# 移動平均計算に使用した関数：<function moving_average_batch_python_online at 0x7f656a23fa60>
# 移動平均の長さ：5000
# 移動平均の最後の要素：1428216.9284
# csvロードにかかった時間：0:00:00.750074秒
# 移動平均計算にかかった時間：0:00:03.029924秒
