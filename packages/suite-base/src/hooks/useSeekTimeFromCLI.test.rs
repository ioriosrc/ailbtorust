```rust
use super::MessagePipeline;
use super::AppParametersContext;
use super::PlayerPresence;
use super::parse_multiple_times;

#[allow(unused_imports)]
use crate::utils::TimestampConversionError;
use std::sync::mpsc::{channel, Receiver};

pub async fn use_seek_time_from_cli() -> (Receiver<()>, ()> {
    let mut receiver = channel::<()>();
    let sender = receiver.clone();

    tokio::spawn(async move {
        match super::use_app_parameters().await {
            Ok(app_params) => {
                if app_params.time.is_none() {
                    log::warn!("Time parameter not provided on CLI. Please check and try again.");
                    return;
                }

                let parsed_time = parse_multiple_times(&app_params.time).await.unwrap_or_else(|err| {
                    log::error!(
                        "Invalid time format using 'time' parameter on CLI. Please check and try again.",
                        error = err
                    );
                    return;
                });

                super::use_message_pipeline().await.unwrap_or_else(|err| {
                    log::error!("Failed to enqueue seek playback command: {}", err);
                    return;
                })
            }
            Err(err) => {
                log::error!("Failed to get app parameters: {}", err);
                sender.send(()).unwrap();
                return;
            }
        }

        sender.send(()).unwrap();
    });

    (receiver, ())
}
```