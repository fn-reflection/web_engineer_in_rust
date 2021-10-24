from pathlib import Path
from pandas import DataFrame


def value_from_index(index):
    return float(index // 7 + index % 7)


CHAPTER2_ROOT_PATH = Path(__file__).resolve().parents[1]
CSV_OUTPUT_PATH = CHAPTER2_ROOT_PATH / 'data/time_series.csv'
if __name__ == "__main__":
    indexes = range(1, 10000001)
    values = [value_from_index(index) for index in indexes]
    df = DataFrame({'value': values})
    df.to_csv(CSV_OUTPUT_PATH, index=False)
