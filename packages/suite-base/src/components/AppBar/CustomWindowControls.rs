```rust
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CustomWindowControls {
    is_maximized: bool,
}

async fn minimize_window() -> HttpResponse {
    // Implement the logic to minimize the window
    HttpResponse::Ok().json(CustomWindowControls { is_maximized: false })
}

async fn maximize_window() -> HttpResponse {
    // Implement the logic to maximize the window
    HttpResponse::Ok().json(CustomWindowControls { is_maximized: true })
}

async fn unmaximize_window() -> HttpResponse {
    // Implement the logic to unmaximize the window
    HttpResponse::Ok().json(CustomWindowControls { is_maximized: false })
}

async fn close_window() -> HttpResponse {
    // Implement the logic to close the window
    HttpResponse::Ok()
}
```

Note that this code uses `actix-web` as a framework and doesn't include any specific UI elements like React components or MUI icons. It provides the basic structure for handling minimize, maximize, and close operations in an Actix-web application.