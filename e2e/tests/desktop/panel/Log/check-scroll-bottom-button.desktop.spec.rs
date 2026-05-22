```rust
use web_sys::{Element, ElementRef, HtmlDocument, Window};

async fn change_to_epoch_format(document: &HtmlDocument) {
    let timestamp_input = document.get_element_by_id("PlaybackTime-text").unwrap();
    let current_timestamp = timestamp_input.text_content().unwrap().parse::<i64>().unwrap();
    // Convert to epoch format
    let epoch_timestamp = current_timestamp * 1000;
    timestamp_input.set_attribute("value", &epoch_timestamp.to_string());
}

async fn load_files(main_window: &Window, filenames: &[&str]) {
    for filename in filenames {
        // Load the files logic here
    }
}

#[test]
async fn open_log_panel_after_loading_an_mcap_file() {
    // Given
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let main_window: ElementRef<HtmlDocument> = window.get_element_by_id("main-window").unwrap();

    load_files(&window, &["example_logs.mcap"]).await;

    // When
    await main_window.get_element_by_role("button", { name: "Add Panel Button" }).click().await;
    await main_window.get_element_by_role("button", { name: "Log" }).click().await;
    await main_window.get_element_by_id("log-panel-root").get_by_role("button", { name: "Settings" }).click().await;

    // Then
    let log_panel = main_window.get_element_by_test_id("log-panel-root").unwrap();
    expect(log_panel.text_content()).to_equal("Log panel");
}

#[test]
async fn should_show_scroll_to_bottom_button_when_there_is_a_scroll_up_in_the_log_panel() {
    // This test usually takes slightly longer than the default 30s timeout
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let main_window: ElementRef<HtmlDocument> = window.get_element_by_id("main-window").unwrap();

    load_files(&window, &["example_logs.mcap"]).await;

    // When
    await main_window.get_element_by_role("button", { name: "Add Panel Button" }).click().await;
    await main_window.get_element_by_role("button", { name: "Log" }).click().await;

    let play_button = main_window.get_element_by_test_id("play-button").unwrap();
    change_to_epoch_format(document).await;

    let timestamp_input = main_window.get_element_by_id("PlaybackTime-text").unwrap();
    let current_timestamp = timestamp_input.text_content().unwrap().parse::<i64>().unwrap();

    await play_button.click().await;

    // Verify timestamp actually moves.
    let mut timeout: u32 = 0;
    let mut interval = async move {
        let current_timestamp = timestamp_input.text_content().unwrap().parse::<i64>().unwrap();
        if current_timestamp > initial_timestamp {
            return Ok(());
        }
        timeout += 1;
        if timeout >= 5 {
            panic!("Timestamp did not move after the play button was clicked");
        }
    };
    loop {
        await interval.await;
    }

    let log_panel = main_window.get_element_by_test_id("log-panel-root").unwrap();
    let scrollTo_bottom_btn = main_window.get_element_by_test_id("scroll-to-bottom-button").unwrap();

    log_panel.hover().await;

    // Scroll up until the button shows up. More resiliant than a single scroll which can be flaky.
    let mut timeout: u32 = 0;
    let mut interval = async move {
        let current_timestamp = timestamp_input.text_content().unwrap().parse::<i64>().unwrap();
        if current_timestamp > initial_timestamp {
            return Ok(());
        }
        timeout += 1;
        if timeout >= 10 {
            panic!("Timestamp did not move after the play button was clicked");
        }
    };
    loop {
        await interval.await;
    };

    // Ensure the button is visible
    expect(scroll_to_bottom_btn.is_visible()).await;
}
```