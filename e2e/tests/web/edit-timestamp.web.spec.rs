```rust
use playwright::{test, browser::BrowserContext};
use std::path::Path;

async fn load_files(context: &BrowserContext) {
    context.navigate_to("http://localhost:3000");
    context.download_file(Path::new("examples/example.mcap"));
}

#[tokio::test]
async fn test_manual_edit_timestamp_playback_time_display(player: BrowserContext) {
    let time_param = |page| page.url().to_string().split('?').last().unwrap().split('=').nth(1).unwrap();
    let url_initial_timestamp = "2025-02-26T10:37:15.547000000Z";

    load_files(&player).await;
    assert_eq!(time_param(&player), url_initial_timestamp);

    let timestamp_input = player.locator("input#PlaybackTime-text").first();
    let new_timestamp = "2025-02-26 10:37:18.499 AM WET";
    timestamp_input.fill(new_timestamp);
    await timestamp_input.press("Enter");

    let url_new_timestamp = "2025-02-26T10:37:18.499000000Z";
    assert_eq!(time_param(&player), url_new_timestamp);
}

#[tokio::test]
async fn test_manual_edit_timestamp_playback_time_display_epoch(player: BrowserContext) {
    let time_param = |page| page.url().to_string().split('?').last().unwrap().split('=').nth(1).unwrap();
    let url_initial_timestamp = "2025-02-26T10:37:15.547000000Z";

    load_files(&player).await;
    player.press_key("Tab");
    player.click("#playback-time-display-toggle-button");
    player.click("#playback-time-display-option-SEC");

    let timestamp_input = player.locator("input#PlaybackTime-text").first();
    let new_epoch_timestamp = "1740566238.499000000";
    timestamp_input.fill(new_epoch_timestamp);
    await timestamp_input.press("Enter");

    let url_new_timestamp = "2025-02-26T10:37:18.499000000Z";
    assert_eq!(time_param(&player), url_new_timestamp);
}
```

Note that this code is a high-level implementation and may need adjustments based on the actual structure of your Playwright project and the specific requirements of your Rust environment.