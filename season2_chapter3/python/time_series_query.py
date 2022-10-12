
from typing import Union
import pandas as pd
import polars as pl


def by_pandas(df: pd.DataFrame, window_size: int):
    """pandasを用いて移動平均を計算する"""
    features = df.rolling(window_size).mean()
    return features


def by_polars(df: Union[pl.DataFrame, pl.LazyFrame], window_size: int):
    """polarsを用いて移動平均を計算する"""
    features = df.select(
        [
            pl.col('value').rolling_mean(window_size),
        ]
    )
    return features


if __name__ == '__main__':
    # pandasでの特徴量計算
    series_by_pandas = by_pandas(pd.read_csv('../data/time_series.csv'), 50)
    print(series_by_pandas)
    # polarsでの特徴量計算(遅延評価しない)
    series_by_eager_polars = by_polars(pl.read_csv('../data/time_series.csv'), 50)
    print(series_by_eager_polars)
    # polarsでの特徴量計算(遅延評価する)
    series_by_lazy_polars = by_polars(pl.scan_csv('../data/time_series.csv'), 50).collect()
    print(series_by_lazy_polars)
