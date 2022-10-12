from typing import Union
import numpy as np
import pandas as pd
import polars as pl


def by_pandas(df: pd.DataFrame):
    """pandasを用いてアヤメの種類ごとの特徴量を抽出する"""
    features = (
        df
        .groupby('class')  # アヤメの種類で集約する
        # pandasではマルチインデックスがサポートされているのでこう書ける
        .agg({
            'sepal_length': [np.mean, np.std],
            'sepal_width': [np.mean, np.std],
            'petal_length': [np.mean, np.std],
            'petal_width': [np.mean, np.std],
        })
        # がく片の長さの平均値で表を並び替える(降順)
        .sort_values(by=[('sepal_length', 'mean')], ascending=False)
    )
    return features


def by_polars(df: Union[pl.DataFrame, pl.LazyFrame]):
    """polarsを用いてアヤメの種類ごとの特徴量を抽出する"""
    features = (
        df
        .groupby('class')  # アヤメの種類で集約する
        .agg(
            [
                pl.count(),  # アヤメの種類ごとの測定数
                # polarsはマルチインデックスを現在サポートしていない
                # リスト内包表記などを使うことで抽象化は可能
                pl.col('sepal_length').mean().alias('sepal_length_mean'),
                pl.col('sepal_length').std().alias('sepal_length_std'),
                pl.col('sepal_width').mean().alias('sepal_width_mean'),
                pl.col('sepal_width').std().alias('sepal_width_std'),
                pl.col('petal_length').mean().alias('petal_length_mean'),
                pl.col('petal_length').std().alias('petal_length_std'),
                pl.col('petal_width').mean().alias('petal_width_mean'),
                pl.col('petal_width').std().alias('petal_width_std'),
            ]
        )
        # がく片の長さの平均値で表を並び替える(降順)
        .sort('sepal_length_mean', reverse=True)
    )
    return features


if __name__ == '__main__':
    # pandasでの特徴量計算
    features_by_pandas = by_pandas(pd.read_csv('../data/irisx10000.csv'))
    print(features_by_pandas)
    # polarsでの特徴量計算(遅延評価しない)
    features_by_eager_polars = by_polars(pl.read_csv('../data/irisx10000.csv'))
    print(features_by_eager_polars)
    # polarsでの特徴量計算(遅延評価する)
    features_by_lazy_polars = by_polars(pl.scan_csv('../data/irisx10000.csv')).collect()
    print(features_by_lazy_polars)
