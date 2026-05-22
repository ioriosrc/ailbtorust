```rust
use playwright::api::{Page, Locator};
use std::time::Instant;

async fn change_to_epoch_format(main_window: Page) -> Result<(), Box<dyn std::error::Error>> {
    let initial_time_in_utc = "2025-02-26 10:37:15.547 AM WET";

    // get date values in epoch format
    let player_starting_time = mainWindow.locator("input[value=\"{}\"]", Some(initial_time_in_utc));
    await player_starting_time.hover();
    let timestamp_dropdown = mainWindow.getByTestId("playback-time-display-toggle-button");
    await timestamp_dropdown.click();

    let new_timestamp_option = mainWindow.getByTestId("playback-time-display-option-SEC");
    let start_time = Instant::now();
    while !new_timestamp_option.is_visible() {
        if Instant::now().duration_since(start_time) > std::time::Duration::from_secs(5) { // wait for 5 seconds
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Timestamp option is not visible")));
        }
        await new_timestamp_option.wait();
    }

    new_timestamp_option.click();

    Ok(())
}
```