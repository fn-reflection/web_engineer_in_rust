# Web Engineer In Rust Season2 Chapter1
## 要約
Rustからsqlxを用いてMySQLに接続する方法を示します。
MySQLのdocker管理などを試していますが、環境依存性を完全に取り除けていないので
うまく動かなくてもご容赦ください。

## Rustの実行コマンド
```shell
cargo run --bin init_db # DBのテーブル生成
cargo run --bin web_engineer_in_rust # アヤメデータの読み書きプログラム実行
```
## dockerコンテナ起動
```shell
docker compose up -d
```

## Rustの実行コマンド(dockerコンテナを用いる場合)
```shell
docker compose run --rm rust_web_container /bin/bash # コンテナ内部でbash起動
cargo run --bin init_db # DBのテーブル生成
cargo run --bin web_engineer_in_rust # アヤメデータの読み書きプログラム実行
```

## 参考文献
アヤメのデータセット(iris.data)は下記より引用しております

Dua, D. and Graff, C. (2019). UCI Machine Learning Repository [http://archive.ics.uci.edu/ml]. Irvine, CA: University of California, School of Information and Computer Science.