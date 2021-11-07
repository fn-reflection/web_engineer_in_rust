## 各スクリプトファイルの実行
パッケージ管理にpoetryというライブラリを使っているので、インストールしてください。
https://python-poetry.org/docs/
```
poetry install # 依存ライブラリをインストール
poetry run python run_calc_batch_list.py # pure pythonのバッチ計算プログラムを動かす
poetry run python run_calc_batch_numpy.py # numpyのバッチ計算プログラムを動かす
poetry run python run_calc_stream.py # ストリーム計算プログラムを動かす
```
## ディレクトリの解説  
- run_calc_batch_list.py
  - 純粋なpythonのみで移動平均を計算するバッチスクリプトです

- run_calc_batch_numpy.py
  - numpy, pandas, numbaを利用して移動平均を計算するバッチスクリプトです

- run_calc_stream.py
  - stream風に移動平均を計算した時にどうなるかを表現したスクリプトです

- _generate_time_series.py
  - 時系列データを作るスクリプトです(実行不要)

- _plot_time_series.ipynb
  - 時系列データを移動平均とともに描画するノートブックです(実行不要)
  
- pyproject.toml
  - poetryというパッケージ管理ツールのプロジェクトファイルです
  - これを用いて利用する外部ライブラリの管理をしています

- poetry.lock
  - poetryが生成する外部ライブラリのバージョンリストです

## 注意事項
このnumpyはpipを使ってインストールされているのでopenblasが使われます。
anacondaを使うことでより高速化できるかもしれません。