# Ruitter webUI
## 目的
Rustで作成したSNSアプリケーションRuitterのAPIサーバの主にデバッグ用のwebUI実装です。

## 前提
- ローカルポートを使用: `src/lib/env.ts`で指定しています(8889)、衝突している場合は変更必要。
- Rust APIサーバをlocalhost:8888ポートで待ち受けさせた状態で起動する

## 基本コマンド
```shell
npm install # node_modulesのインストール
npm run dev # localでdevサーバ立ち上げて実行
```

## 利用方法
`npm run dev`で起動後、[http://localhost:8889](http://localhost:8889)にアクセス

### 機能
- ユーザー登録機能(DBに未登録のユーザー名をPOSTすると成功するはず)
- ログイン機能(DBに登録済のユーザー名をPOSTすると成功し、クッキーがセットされる)
- フォロー機能(有効なセッションキーを持つクッキーヘッダーをつけてDBに登録済のユーザー名をPOSTすると成功するはず)
- ツイート機能(有効なセッションキーを持つクッキーヘッダーをつけて140文字以内のテキストをPOSTすると成功するはず)
- タイムライン機能(有効なセッションキーを持つクッキーヘッダーをつけてボタンを押すと、自分とフォローしているユーザのツイートが見れる)