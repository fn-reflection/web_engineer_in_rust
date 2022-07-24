# Web Engineer In Rust Season2 Chapter1
## 要約
RustでSNSアプリケーションをスケッチします。
MySQLのdocker管理などを試していますが、環境依存性を完全に取り除けていないのでうまく動かなくてもご容赦ください。
localhostのTCP53306ポートでMySQLサーバが立ち上がっていると仮定します。
(dockerインストール済の場合は、下記dockerコマンドで再現できます)

[サーバサイド(Rust)のREADME](./server/README.md)

## dockerコンテナ起動
```shell
docker compose up -d # MySQLコンテナとRustコンテナが立ち上がる
```
