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
cargo run --release --bin time_series_query # 時系列データを対象にした窓関数クエリ
```
