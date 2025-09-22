# Rust API ドキュメント

## 概要

このAPIは日本語をサポートするRust言語で書かれたWebAPIサーバーです。RESTfulなエンドポイントを提供し、JSONレスポンスを返します。

## エンドポイント一覧

### 1. ウェルカムメッセージ

**エンドポイント:** `GET /`

**説明:** サーバーのウェルカムメッセージを日本語で返します。

**レスポンス例:**
```json
{
  "message": "Rust APIサーバーへようこそ！日本語をサポートしています。",
  "status": "success",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### 2. サービスステータス

**エンドポイント:** `GET /api/status`

**説明:** APIサーバーの現在のステータス情報を返します。

**レスポンス例:**
```json
{
  "service": "rust-api",
  "version": "0.1.0",
  "status": "正常稼働中",
  "message": "すべてのシステムが正常に動作しています"
}
```

### 3. 挨拶メッセージ

**エンドポイント:** `GET /api/hello`

**説明:** シンプルな挨拶メッセージを日本語で返します。

**レスポンス例:**
```json
{
  "message": "こんにちは！Rustで作られたAPIです。",
  "status": "success",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## レスポンス形式

### 基本レスポンス構造

```json
{
  "message": "メッセージ内容",
  "status": "success|error",
  "timestamp": "ISO8601形式のタイムスタンプ"
}
```

### ステータスレスポンス構造

```json
{
  "service": "サービス名",
  "version": "バージョン",
  "status": "ステータス",
  "message": "詳細メッセージ"
}
```

## エラーハンドリング

APIは適切なHTTPステータスコードとともに、エラー情報を日本語で返します。

## セキュリティ

- CORS設定が有効になっています
- すべてのエンドポイントでJSONレスポンスを返します

## 使用例

### cURLを使用した例

```bash
# ウェルカムメッセージを取得
curl http://localhost:3030/

# サービスステータスを確認
curl http://localhost:3030/api/status

# 挨拶メッセージを取得
curl http://localhost:3030/api/hello
```

### JavaScriptを使用した例

```javascript
// ウェルカムメッセージを取得
fetch('http://localhost:3030/')
  .then(response => response.json())
  .then(data => console.log(data.message));

// サービスステータスを確認
fetch('http://localhost:3030/api/status')
  .then(response => response.json())
  .then(data => console.log(`${data.service}: ${data.status}`));
```

## サポート

このAPIに関するご質問やサポートが必要な場合は、プロジェクトのIssueページをご利用ください。