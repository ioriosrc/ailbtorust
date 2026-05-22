```rust
use std::rc::Rc;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use actix_web::{web, App, HttpResponse};
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    App::new()
        .route("/gauge", web::get().to(gauge))
        .run().expect("Failed to start server");
}

async fn gauge() -> HttpResponse {
    let config = Rc::new(serde_json::from_str("{\"path\": \"your_topic\"}").unwrap());
    Ok(web::Json(config))
}
```