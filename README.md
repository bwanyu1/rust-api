# Rust API

日本語対応のRust APIサーバーです。

## 概要

このプロジェクトは、日本語をサポートするRust言語で書かれたWebAPIサーバーです。

## 機能

- RESTful API エンドポイント
- JSON レスポンス対応
- 日本語メッセージのサポート
- エラーハンドリング

## セットアップ

### 前提条件

- Rust 1.70以上がインストールされていること
- Cargo パッケージマネージャー

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/bwanyu1/rust-api.git
cd rust-api

# 依存関係をインストール
cargo build

# アプリケーションを実行
cargo run
```

## 使用方法

サーバーを起動すると、以下のエンドポイントが利用可能になります：

- `GET /` - ウェルカムメッセージ（日本語）
- `GET /api/status` - サービスステータス
- `GET /api/hello` - 挨拶メッセージ

## 開発

```bash
# テストを実行
cargo test

# フォーマットを確認
cargo fmt

# リンターを実行
cargo clippy
```

## ライセンス

MIT License