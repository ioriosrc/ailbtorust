```rust
use playwright::{api::BrowserContext, Locator, Page, Playwright};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a browser instance
    let playwright = Playwright::new().await?;
    let browser = playwright.chromium().launch()?;

    // Create a context for browser
    let context = browser.new_context();

    // Open new pages in the context
    let page1 = context.new_page();
    let page2 = context.new_page();

    // Navigate both pages to the Lichtblick web app
    await page1.goto("https://example.com");
    await page2.goto("http://localhost:8080");

    // Wait for both pages to load
    await page1.waitFor_load_state("networkidle");
    await page2.waitFor_load_state("networkidle");

    // Load the same file in both instances
    let filename = "example.mcap";

    // Load file in first tab
    await load_files(&page1, vec![filename]);

    // Load file in second tab
    await load_files(&page2, vec![filename]);

    // Verify sync button is available and initially off in both tabs
    await expect(page1.locator("input[value*='2025-02-26']")).to_be_visible();
    await expect(page2.locator("input[value*='2025-02-26']")).to_be_visible();
    await expect(page1.locator("button[value*='off']").to_be_enabled());
    await expect(page2.locator("button[value*='off']").to_be_enabled());

    // Enable sync in both instances by clicking the Sync button
    await page1.locator("button[value*='Sync']").click();
    await page2.locator("button[value*='Sync']").click();

    // Verify sync is enabled (button should show "on")
    await expect(page1.locator("button[value*='on']").to_be_enabled());
    await expect(page2.locator("button[value*='on']").to_be_enabled());

    // Get initial timestamps to ensure they're the same before playing
    let initial_time_input1 = page1.locator("input[value*='2025-02-26']").first();
    let initial_time_input2 = page2.locator("input[value*='2025-02-26']").first();

    let initial_time1 = initial_time_input1.input_value()?;
    let initial_time2 = initial_time_input2.input_value()?;

    // Both instances should start with the same timestamp
    assert_eq!(initial_time1, initial_time2);

    // Click play on the first instance
    await page1.locator("button[value*='Play']").click();
    await page1.wait_for_timeout(100);

    // Verify both instances are playing (pause button visible)
    await expect(page1.locator("button[value*='Pause']").to_be_visible());
    await expect(page2.locator("button[value*='Pause']").to_be_visible());

    // Wait a bit for sync to propagate and playback to advance
    await page1.wait_for_timeout(2000);

    // Pause to get stable timestamps
    await page1.locator("button[value*='Pause']").click();
    await page1.wait_for_timeout(100);

    // Verify both instances are paused (play button visible)
    await expect(page1.locator("button[value*='Play']").to_be_visible());
    await expect(page2.locator("button[value*='Play']").to_be_visible());

    // Get current time from both instances after playing
    let time_input1 = page1.locator("input[value*='2025-02-26']").first();
    let time_input2 = page2.locator("input[value*='2025-02-26']").first();

    let time1 = time_input1.input_value()?;
    let time2 = time_input2.input_value()?;

    // Both instances should have timestamps within 100ms of each other (allowing for sync delay)
    // Parse timestamps and compare
    let timestamp1 = parse_timestamp(time1);
    let timestamp2 = parse_timestamp(time2);

    let time_diff = (timestamp1 as f64 - timestamp2 as f64).abs();

    assert!(time_diff < 0.1); // Allow up to 100ms difference for sync propagation

    // The timestamp should have advanced from the initial time
    assert_ne!(time1, initial_time1);

    // Test seek synchronization
    // Seek forward on instance 1
    await page1.locator("button[value*='Seek forward']").click();
    await page1.wait_for_timeout(100);

    let new_time1 = time_input1.input_value()?;
    let new_time2 = time_input2.input_value()?;

    // Timestamps should have changed and still be synchronized
    assert_ne!(new_time1, time1);

    let new_timestamp1 = parse_timestamp(new_time1);
    let new_timestamp2 = parse_timestamp(new_time2);
    let new_time_diff = (new_timestamp1 as f64 - new_timestamp2 as f64).abs();

    assert!(new_time_diff < 0.1); // Allow up to 100ms difference for sync propagation

    await page1.close();
    await page2.close();
    await context.close();

    Ok(())
}
```