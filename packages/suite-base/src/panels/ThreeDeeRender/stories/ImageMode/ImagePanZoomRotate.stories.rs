```rust
use actix_web::{web, App, HttpResponse};
use std::net::SocketAddr;

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .set_header("Content-Type", "text/html; charset=utf-8")
        .body(
            r#"
<html>
<head>
    <title>Image Panel</title>
</head>
<body>
    <canvas id="panel" width="600" height="450"></canvas>
    <script src="https://cdn.jsdelivr.net/npm/three@0.127.0/build/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/@lichtblick/suite-base/panels/ThreeDeeRender/stories/ImageMode/imageCommon.js"></script>
    <script src="src/index.tsx"></script>
</body>
</html>
"#,
        )
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(addr)?
    .run()
    .await
}
```

Este código implementa um servidor HTTP usando `actix-web` que fornece uma página HTML contendo um canvas e a estrutura necessária para renderizar imagens, incluindo um botão de panagem. Ele também carrega o JavaScript necessário para manipular o mouse e as interações do usuário com o canvas.