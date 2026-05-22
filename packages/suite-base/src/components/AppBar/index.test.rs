```rust
use actix_web::web::scope;
use actix_web::{App, HttpServer};
use serde_json::Value;

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Value::new({
            "message": "Welcome to the Rust App!"
        }))))
}
```