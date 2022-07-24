## Rustの実行コマンド
```shell
cargo run --bin init_db # DBのテーブル生成
cargo run --bin ruitter # ruitter APIサーバの起動
cargo test -- --test-threads=1 # テストの実行
```

## APIサーバの動作検証に有用なコマンド
```shell
curl -X POST -H "Content-Type: application/json" -d '{"name":"abcdeff"}' http://localhost:8888/users # ユーザ新規作成挙動の確認
curl -v -X POST -H "Content-Type: application/json" -d '{"name":"abcdeffgg"}' http://localhost:8888/sessions # ログイン挙動の確認
```


## Rustの実行コマンド(dockerコンテナを用いる場合)
```shell
# ソースコードのDB_STRING_PRODUCTIONをコメントアウトされているものに書き換える
docker compose run --rm rust_web_container /bin/bash # コンテナ内部でbash起動
cargo run --bin init_db # DBのテーブル生成
cargo run --bin ruitter # アヤメデータの読み書きプログラム実行
```
