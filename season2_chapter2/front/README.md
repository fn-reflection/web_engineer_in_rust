# Unused Code Visualizer
## 目的
DynamoDBに蓄積されている不要コード情報を可視化するツールです。

## 前提
- ローカルポートを使用: `src/lib/env.ts`で指定しています(8889)、衝突している場合は変更必要。
- Rust APIサーバを8888ポートで待ち受けさせた状態で起動する

## 基本コマンド
```shell
npm install # node_modulesのインストール
npm run dev # localでdevサーバ立ち上げて実行
```

## 利用方法
`npm run dev`で起動後、[http://localhost:8889](http://localhost:8889)にアクセス
