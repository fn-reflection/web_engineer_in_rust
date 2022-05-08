## Rustの実行コマンド
```shell
cargo run --bin init_db # DBのテーブル生成
cargo run --bin web_engineer_in_rust # アヤメデータの読み書きプログラム実行
```

## 便利なdockerコマンド
```shell
docker compose up -d # コンテナの起動
docker compose build --no-cache --pull # コンテナをrebuild
docker compose run --rm rust_web_container /bin/bash # コンテナ内部でbash起動
```

## 参考文献
アヤメのデータセット(iris.data)は下記より引用しております

Dua, D. and Graff, C. (2019). UCI Machine Learning Repository [http://archive.ics.uci.edu/ml]. Irvine, CA: University of California, School of Information and Computer Science.