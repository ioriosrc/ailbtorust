```rust
use playwright::{api::Locator, browser_context::BrowserContext, dialog::Dialog, Page};

async fn move_player_time(page: &Page, direction: &str, duration_ms: i32) {
    let mut input_value = page.locator(`input[value="${direction}"]`);
    for _ in 0..duration_ms / 10 {
        input_value = input_value.evaluate_with_timeout(move || {
            if let Ok(time_input) = input_value.text_content() {
                // Update the time by adding or subtracting the duration
                let current_time = time_input.parse::<f64>().unwrap();
                let new_time = if direction == "forward" {
                    current_time + (duration_ms as f64 / 1000.0)
                } else {
                    current_time - (duration_ms as f64 / 1000.0)
                };
                time_input.set_value(new_time.to_string());
            }
        }, 2000).await.unwrap();
    }
}

#[test]
async fn update_step_size_via_settings_and_verify_change_on_player(page: &Page) {
    // Given
    const initial_time = "2025-02-26 10:37:15.547 AM WET";
    const forwarded_time = "2025-02-26 10:37:15.947 AM WET";

    let filename = "example.mcap";
    await page.goto("http://localhost:8080");

    // Then
    let player_starting_time = page.locator(`input[value="${initial_time}"]`);
    assert_eq!(await player_starting_time.get_input_value(), initial_time);

    // When
    await page.click("#user-button");
    await page.click("Visualization settings");
    await page.fill("#stepSizeInput", "400");
    await page.click("Done");

    await move_player_time(&page, "forward", 400);
    let player_forwarded_time = page.locator(`input[value="${forwarded_time}"]`);
    assert_eq!(await player_forwarded_time.get_input_value(), forwarded_time);

    // When
    await move_player_time(&page, "backward", 400);
    assert_eq!(await player_starting_time.get_input_value(), initial_time);
}
```