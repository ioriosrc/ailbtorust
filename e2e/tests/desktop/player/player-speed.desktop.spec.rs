```rust
use web_sys::window;

fn main() {
    let window = web_sys::window().expect("Could not find the window");
    let document = &window.document().expect("Could not find the document");

    // Given a .mcap file is loaded
    let mcap_filename = "example.mcap";

    // WHEN playback speed is set to 2x
    set_playback_speed(document, "2×", 500);

    // THEN it should play roughly twice as fast
    assert_speed_ratio(document, 2.0, 500);

    // GIVEN a .mcap file is loaded
    set_playback_speed(document, "0.1×", 500);

    // WHEN playback speed is set to 0.1x
    assert_speed_ratio(document, 0.1, 500);
}

fn set_playback_speed(doc: &web_sys::Document, speed_option: &str, duration_ms: u32) {
    let dropdown_menu = doc.get_element_by_id("PlaybackSpeedControls-Dropdown").unwrap();
    let items = dropdown_menu.get_elements_by_role("menuitem").unwrap();

    for item in items.iter() {
        if item.text_content() == speed_option {
            item.click().expect("Could not click the menu item");
            break;
        }
    }

    let start_time = window.performance.now().as_f64();
    while (window.performance.now().as_f64() - start_time) < duration_ms as f64 {
        // Simulate playback
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let end_time = window.performance.now().as_f64();
    assert!(end_time > start_time, "Playback did not finish in the specified duration");
}

fn assert_speed_ratio(doc: &web_sys::Document, expected_ratio: f64, duration_ms: u32) {
    let timestamp_input = doc.get_element_by_id("PlaybackTime-text").unwrap().get_element_by_role("input").unwrap();
    let start_time = window.performance.now().as_f64();

    while (window.performance.now().as_f64() - start_time) < duration_ms as f64 {
        // Simulate playback
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let end_time = window.performance.now().as_f64();
    let elapsed_time = end_time - start_time;

    let progress = (elapsed_time / duration_ms as f64) * 100.0;
    println!("Progress: {:.2}%", progress);

    assert!(progress > expected_ratio * 0.8 && progress < expected_ratio * 1.2, "Playback speed is not within the expected range");
}
```