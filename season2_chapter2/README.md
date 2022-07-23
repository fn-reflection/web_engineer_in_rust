# Web Engineer In Rust Season2 Chapter1
## 要約
RustでSNSアプリケーションをスケッチします。
MySQLのdocker管理などを試していますが、環境依存性を完全に取り除けていないのでうまく動かなくてもご容赦ください。
localhostのTCP53306ポートでMySQLサーバが立ち上がっていると仮定します。
(dockerインストール済の場合は、下記dockerコマンドで再現できます)

## Rustの実行コマンド
```shell
cargo run --bin init_db # DBのテーブル生成
cargo run --bin web_engineer_in_rust # 
cargo test -- --test-threads=1 # テストの実行
```
## dockerコンテナ起動
```shell
docker compose up -d # MySQLコンテナとRustコンテナが立ち上がる
```

## APIサーバの動作検証に有用なコマンド
```shell
curl -X POST -H "Content-Type: application/json" -d '{"name":"abcdeff"}' http://localhost:8888/users
curl -v -X POST -H "Content-Type: application/json" -d '{"name":"abcdeffgg"}' http://localhost:8888/sessions
```


## Rustの実行コマンド(dockerコンテナを用いる場合)
```shell
# ソースコードのDB_STRING_PRODUCTIONをコメントアウトされているものに書き換える
docker compose run --rm rust_web_container /bin/bash # コンテナ内部でbash起動
cargo run --bin init_db # DBのテーブル生成
cargo run --bin web_engineer_in_rust # アヤメデータの読み書きプログラム実行
```
