# web engineer in Rust
## command
### build option
```
cargo run --release --bin run_calc_batch # バッチ計算プログラムを動かす
cargo run --release --bin run_calc_stream # ストリーム計算プログラムを動かす
cargo run --release --bin run_calc_stream_channel # ストリーム計算プログラム(thread + channel)を動かす
cargo run --release --bin run_calc_stream_async # ストリーム計算プログラム(async with tokio)を動かす
```
