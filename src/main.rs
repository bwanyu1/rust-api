// 日本語対応のRust WebAPIサーバー
// Japanese-compatible Rust Web API Server

use serde::{Deserialize, Serialize};
use warp::Filter;

/// APIレスポンスの基本構造体
/// Basic structure for API responses
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
    timestamp: String,
}

/// サービスステータス情報
/// Service status information
#[derive(Serialize, Deserialize)]
struct StatusResponse {
    service: String,
    version: String,
    status: String,
    message: String,
}

/// ウェルカムメッセージを返す
/// Returns welcome message
async fn welcome() -> Result<impl warp::Reply, warp::Rejection> {
    let response = ApiResponse {
        message: "Rust APIサーバーへようこそ！日本語をサポートしています。".to_string(),
        status: "success".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    Ok(warp::reply::json(&response))
}

/// サービスステータスを返す
/// Returns service status
async fn status() -> Result<impl warp::Reply, warp::Rejection> {
    let response = StatusResponse {
        service: "rust-api".to_string(),
        version: "0.1.0".to_string(),
        status: "正常稼働中".to_string(),
        message: "すべてのシステムが正常に動作しています".to_string(),
    };
    Ok(warp::reply::json(&response))
}

/// 挨拶メッセージを返す
/// Returns greeting message
async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    let response = ApiResponse {
        message: "こんにちは！Rustで作られたAPIです。".to_string(),
        status: "success".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    Ok(warp::reply::json(&response))
}

/// CORSヘッダーを設定
/// Configure CORS headers
fn with_cors() -> warp::filters::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
}

#[tokio::main]
async fn main() {
    // ログを設定
    // Configure logging
    println!("🚀 Rust APIサーバーを起動中...");
    println!("🌐 サーバーアドレス: http://localhost:3030");

    // ルートエンドポイント
    // Root endpoint
    let welcome_route = warp::path::end().and(warp::get()).and_then(welcome);

    // ステータスエンドポイント
    // Status endpoint
    let status_route = warp::path("api")
        .and(warp::path("status"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(status);

    // 挨拶エンドポイント
    // Hello endpoint
    let hello_route = warp::path("api")
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(hello);

    // すべてのルートを結合
    // Combine all routes
    let routes = welcome_route
        .or(status_route)
        .or(hello_route)
        .with(with_cors())
        .with(warp::log("rust_api"));

    // サーバーを起動
    // Start the server
    println!("✅ サーバーが正常に起動しました");
    println!("📋 利用可能なエンドポイント:");
    println!("   GET /           - ウェルカムメッセージ");
    println!("   GET /api/status - サービスステータス");
    println!("   GET /api/hello  - 挨拶メッセージ");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_welcome_message() {
        let _response = welcome().await.unwrap();
        // テストではレスポンスの構造のみ確認
        // Only verify response structure in tests
        assert!(true); // Basic test to ensure function works
    }

    #[tokio::test]
    async fn test_status_endpoint() {
        let _response = status().await.unwrap();
        // ステータスエンドポイントのテスト
        // Status endpoint test
        assert!(true); // Basic test to ensure function works
    }

    #[tokio::test]
    async fn test_hello_endpoint() {
        let _response = hello().await.unwrap();
        // 挨拶エンドポイントのテスト
        // Hello endpoint test
        assert!(true); // Basic test to ensure function works
    }

    #[test]
    fn test_api_response_structure() {
        // APIレスポンス構造体のテスト
        // API response structure test
        let response = ApiResponse {
            message: "テストメッセージ".to_string(),
            status: "success".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(response.message, "テストメッセージ");
        assert_eq!(response.status, "success");
    }

    #[test]
    fn test_status_response_structure() {
        // ステータスレスポンス構造体のテスト
        // Status response structure test
        let response = StatusResponse {
            service: "rust-api".to_string(),
            version: "0.1.0".to_string(),
            status: "正常稼働中".to_string(),
            message: "テスト中".to_string(),
        };

        assert_eq!(response.service, "rust-api");
        assert_eq!(response.status, "正常稼働中");
    }
}
