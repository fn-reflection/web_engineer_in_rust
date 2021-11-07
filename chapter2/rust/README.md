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
cargo run --release --bin run_calc_batch # バッチ計算プログラムを動かす
cargo run --release --bin run_calc_stream # ストリーム計算プログラムを動かす
cargo run --release --bin run_calc_stream_channel # ストリーム計算プログラム(thread + channel)を動かす
cargo run --release --bin run_calc_stream_async # ストリーム計算プログラム(async with tokio)を動かす
```
