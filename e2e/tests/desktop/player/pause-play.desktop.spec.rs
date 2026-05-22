```rust
use playwright::{page::Page, Locator};

async fn get_playback_elements(main_window: &Page) -> (Locator, Locator) {
    let button = main_window.locator("#play-button");
    let timestamp = main_window.locator("input#PlaybackTime-text").locator("input");

    (button, timestamp)
}

#[test]
async fn should_start_playing_when_clicking_on_play_button() {
    // Given
    let mut page = Page::new().await?;
    await load_files(&mut page, &["example.mcap"]);
    await change_to_epoch_format(&mut page);

    let (button, timestamp) = get_playback_elements(&page).await;

    // When
    await button.click(); // start playback

    // Then
    assert_eq!(button.get_attribute("title"), Some("Pause"));
    let elapsed_timestamp: i64 = timestamp.input_value().await.parse()?;
    assert!(elapsed_timestamp > 0);
}

#[test]
async fn should_start_playing_when_pressing_spacebar_key() {
    // Given
    let mut page = Page::new().await?;
    await load_files(&mut page, &["example.mcap"]);
    await change_to_epoch_format(&mut page);

    let (button, timestamp) = get_playback_elements(&page).await;

    // When
    assert_eq!(button.get_attribute("title"), Some("Play"));
    await page.keyboard.press("Space"); // start playback

    // Then
    assert_eq!(button.get_attribute("title"), Some("Pause"));
    let elapsed_timestamp: i64 = timestamp.input_value().await.parse()?;
    assert!(elapsed_timestamp > 0);
}

#[test]
async fn should_stop_playing_when_clicking_on_play_button() {
    // Given
    let mut page = Page::new().await?;
    await load_files(&mut page, &["example.mcap"]);
    await change_to_epoch_format(&mut page);

    let (button, timestamp) = get_playback_elements(&page).await;

    // When
    button.click(); // start playback

    // Then
    assert_eq!(button.get_attribute("title"), Some("Pause"));
    button.click(); // stop playback

    // Check if icon has changed first
    assert_eq!(button.get_attribute("title"), Some("Play"));

    let elapsed_timestamp: i64 = timestamp.input_value().await.parse()?;
    assert_eq!(elapsed_timestamp, 0);
}

#[test]
async fn should_stop_playing_when_pressing_spacebar_key() {
    // Given
    let mut page = Page::new().await?;
    await load_files(&mut page, &["example.mcap"]);
    await change_to_epoch_format(&mut page);

    let (button, timestamp) = get_playback_elements(&page).await;

    // When
    button.click(); // start playback

    // Then
    assert_eq!(button.get_attribute("title"), Some("Pause"));
    page.keyboard.press("Space"); // stop playback

    // Check if icon has changed first
    assert_eq!(button.get_attribute("title"), Some("Play"));

    let elapsed_timestamp: i64 = timestamp.input_value().await.parse()?;
    assert_eq!(elapsed_timestamp, 0);
}
```