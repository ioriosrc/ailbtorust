```rust
use actix_web::{web, HttpResponse};
use serde_json::Value;

async fn handle_request(req: web::HttpRequest) -> HttpResponse {
    // Handle your request here
    Ok(HttpResponse::Ok().body("Hello, World!"))
}

fn main() {
    web::serve(|| {
        web::get("/").to(handle_request);
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
```