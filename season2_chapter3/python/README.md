## 各スクリプトファイルの実行
パッケージ管理にpoetryというライブラリを使っているので、インストールしてください。
https://python-poetry.org/docs/

```shell
poetry install # 依存ライブラリをインストール
poetry run python aggregate_query.py # アヤメのデータセットを対象にした集計クエリ
poetry run python time_series_query.py # 時系列データを対象にした窓関数クエリ
poetry run ipython # インタラクティブシェルipythonを起動
```