## Rustの実行コマンド
```shell
cargo run --bin init_db # DBのテーブル生成
cargo run --bin ruitter # ruitter APIサーバの起動
cargo test -- --test-threads=1 # テストの実行
```

## APIサーバの動作検証に有用なコマンド
```shell
curl -X POST -H "Content-Type: application/json" -d '{"name":"test123"}' http://localhost:8888/api/users # ユーザ新規作成挙動の確認
curl -X POST -H "Content-Type: application/json" -d '{"name":"test123"}' -c cookie.txt http://localhost:8888/api/sessions # ログイン挙動とCookieの保存
curl -X POST -H "Content-Type: application/json" -d '{"content":"some tweet"}' -b cookie.txt http://localhost:8888/api/user_tweets # Cookieを使用してメモ作成
curl -X POST -H "Content-Type: application/json" -d '{"name":"test123"}' -b cookie.txt http://localhost:8888/api/follow_relations # Cookieを使用してフォロー
curl -H "Content-Type: application/json" -b cookie.txt http://localhost:8888/api/pages/timeline # Cookieを使用してタイムライン取得
```


## Rustの実行コマンド(dockerコンテナを用いる場合)
```shell
# ソースコードのDB_STRING_PRODUCTIONをコメントアウトされているものに書き換える
docker compose run --rm rust_web_container /bin/bash # コンテナ内部でbash起動
cargo run --bin init_db # DBのテーブル生成
cargo run --bin ruitter # アヤメデータの読み書きプログラム実行
```
