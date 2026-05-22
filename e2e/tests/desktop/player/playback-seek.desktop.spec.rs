```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use playwright::Page;
use std::time::{Duration, Instant};

async fn click_playback_slider(main_window: Page, fraction: f64) {
    let slider = mainWindow.get_by_test_id("playback-slider");
    let timestamp = mainWindow.get_by_test_id("PlaybackTime-text").locator("input");
    let box_ = slider.boundingBox().await.unwrap();
    if box_.is_none() {
        panic!("Slider bounding box not found");
    }

    // Add small offsets for edge cases to ensure click is within bounds
    let offset = 2; // pixels from edge
    let x = if fraction == 0.0 {
        box_.x + offset
    } else if fraction == 1.0 {
        box_.x + box_.width - offset
    } else {
        box_.x + box_.width * fraction;
    };
    let y = box_.y + box_.height / 2;

    await mainWindow.mouse.click(x, y);
}

async fn wait_timestamp(timestamp: Locator) -> Duration {
    let start_time = Instant::now();
    let mut last_value = -1.0;
    while !timestamp.input_value().await.as_f64().unwrap_or(-1.0).abs_diff(last_value) < 0.001 {
        await std::time::sleep(Duration::from_millis(50));
        if start_time.elapsed() > Duration::from_secs(20) {
            return Duration::from_secs(20);
        }
    }

    start_time.elapsed()
}

#[tokio::test]
async fn should_advance_timestamp_100ms_when_seek_forward_button_is_clicked() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let button = main_window.get_by_test_id("seek-forward-button").await?;
    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    button.click();
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(100.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_advance_timestamp_100ms_when_right_arrow_key_is_pressed() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    mainWindow.keyboard.press("ArrowRight").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(100.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_advance_timestamp_500ms_when_alt_right_arrow_key_is_pressed() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    mainWindow.keyboard.down("Alt").await;
    mainWindow.keyboard.press("ArrowRight").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(500.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_regress_timestamp_100ms_when_seek_forward_backward_is_clicked() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let button = main_window.get_by_test_id("seek-backward-button").await?;
    click_playback_slider(main_window, 0.5).await?;
    let timestamp = mainWindow
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    button.click();
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(100.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_regress_timestamp_100ms_when_left_arrow_key_is_pressed() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    mainWindow.keyboard.press("ArrowLeft").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(100.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_regress_timestamp_500ms_when_alt_left_arrow_key_is_pressed() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    mainWindow.keyboard.down("Alt").await;
    mainWindow.keyboard.press("ArrowLeft").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(500.0) < 10.0);

    Ok(())
}

#[tokio::test]
async fn should_foward_timestamp_to_end_of_slider_when_alt_right_arrow_key_is_pressed_less_than_500ms_from_the_end() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let button = main_window.get_by_test_id("seek-backward-button").await?;
    click_playback_slider(main_window, 1.0).await?;
    let timestamp = mainWindow
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    click_playback_slider(main_window, 0.9).await?;

    mainWindow.keyboard.down("Alt").await;
    mainWindow.keyboard.press("ArrowRight").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(0.01) < 0.01);
}

#[tokio::test]
async fn should_regress_timestamp_to_start_of_slider_alt_left_arrow_key_is_pressed_less_than_500ms_from_the_start() -> Result<(), Box<dyn std::error::Error>> {
    let (main_window, _) = Page::new().await?;
    load_files(&main_window, &[MCAP_FILENAME]).await?;

    let timestamp = main_window
        .get_by_test_id("PlaybackTime-text")
        .locator("input")
        .await?
        .value().await;

    click_playback_slider(main_window, 0.0).await?;
    click_playback_slider(main_window, 0.1).await?;

    mainWindow.keyboard.down("Alt").await;
    mainWindow.keyboard.press("ArrowLeft").await;
    let elapsed_time = wait_timestamp(timestamp).await?;

    let current_value = timestamp.value().await;
    let diff = (current_value as f64) - start_time.elapsed().as_secs_f64() * 1000.0;
    assert!(diff.abs_diff(0.01) < 0.01);
}
```