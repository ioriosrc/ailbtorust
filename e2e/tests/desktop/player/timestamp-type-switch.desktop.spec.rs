```rust
use test::*;
use expect::*;
use async_test::*;

#[tokio::test]
async fn should_switch_playback_time_to_epoch_format_next_to_the_player() {
    // Given
    let initial_time_in_utc = "2025-02-26 10:37:15.547 AM WET";
    let intial_time_in_epoch = "1740566235.547000000";

    let filename = "example.mcap";
    load_files(&filename).await;

    // When
    let player_starting_time = await mainWindow.locator("input[value=\"${initial_time_in_utc}\"]");
    // Playback time display needs to be hovered first so clicking on it is possible
    await player_starting_time.hover();

    let timestamp_dropdown = await mainWindow.getByTestId("playback-time-display-toggle-button");
    await timestamp_dropdown.click();

    let new_timestamp_option = await mainWindow.getByTestId("playback-time-display-option-SEC");
    await new_timestamp_option.click();

    // Then
    let player_starting_time_in_epoch = await mainWindow.locator("input[value=\"${intial_time_in_epoch}\"]");
    await expect(player_starting_time_in_epoch).toBe_visible();
}
```