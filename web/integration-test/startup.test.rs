```rust
use std::net::TcpListener;
use std::str;

use http::Response;
use hyper::{body::Body, Request};
use hyper::StatusCode;
use playwright::api::BrowserContextOptions;
use playwright::browser::LaunchOptions;
use playwright::page::Page;
use server_handler::serve_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Running at http://localhost:8080");

    let public_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(".webpack");

    let server = TcpListener::bind("127.0.0.1:8080")?;
    let handle = tokio::spawn(async move {
        while let Some(stream) = server.accept().await {
            log::info!("New connection");
            hyper::service::ServiceBuilder::new(|req| async move {
                serve_handler(req, &public_path).await
            })
            .serve_connection(stream)
            .await;
        }
    });

    let browser_context_options: BrowserContextOptions = Default::default();
    let launch_options: LaunchOptions = Default::default();

    let browser = playwright::chromium::launch(launch_options)?;
    let page = browser.new_page(&browser_context_options)?;

    let url = format!("http://localhost:8080");
    await page.goto(url);

    let mut response = http::Request::get(url);
    while !response.status().is_success() {
        sleep(Duration::from_secs(1));
        response = hyper::Request::get(url);
    }

    let body = response.body();
    if let Ok(body) = body {
        let text = str::from_utf8(&body).await?;
        if text.contains("App rendered") {
            log::info!("App rendered");
        } else {
            log::error!("App failed to render");
        }
    }

    browser.close().await?;

    handle.await?;

    Ok(())
}
```