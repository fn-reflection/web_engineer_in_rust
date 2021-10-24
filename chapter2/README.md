# Web Engineer In Rust Chapter2
## 要約
Rustのパフォーマンスをpythonと比較します。
普通に比較するだけでは、Rustが勝ってしまうことは自明のように思えますが、バッチ計算の文脈においては、pythonの強力なエコシステムを活用することでRustと勝負ができることを示します。一方でストリーム計算の文脈においては、pythonの動的型付け的性質により最適化をかけることが難しく、Rustに優位性があることを示します。
と同時にPythonとRustの構文を対比が取れた構成で表現することにより、Rustがより高度な厳密性が求められるがスクリプト言語に近い記述性を持つことを間接的に示します。
(一方でRustを書く上で必要になるトレイトなどの高度な型抽象は、棚上げにします。)

## README
[PythonのREADME](./python/README.md)
[RustのREADME](./rust/README.md)