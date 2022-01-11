# web engineer in Rust

## 利用ツールについて
rustup: rust自体のバージョン管理ツール
cargo: パッケージ管理を主用途とするRustプロジェクト管理ツール
最低でもこれらのインストールが必要です。(rustupは任意ですが、あったほうが良いです。)
インストール方法の詳細は公式に委ねます。
https://www.rust-lang.org/ja/tools/install

## command
### build option
```
cargo run --release --bin thread_safe_queue # スレッドセーフキューのサンプル実行
cargo run --release --bin moving_average_f64 # トレイト無しの移動平均計算(コンパイルできない)
cargo run --release --bin moving_average_trait # トレイトありの移動平均計算のサンプル実行
cargo run --release --bin use_trait_extension # トレイトによる機能拡張のサンプル実行
```
