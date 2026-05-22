```rust
use playwright::{playwright::Playwright, browser_context::BrowserContext};

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::install().await?;
    let browser = playwright.chromium.launch();
    let context = browser.new_context();

    let mut mainWindow = context.main_window().await?;

    // GIVEN
    let mcap_file = "custom-camera-model.mcap";
    load_files(&mut mainWindow, vec![mcap_file]).await?;

    // WHEN
    await mainWindow.get_by_test_id("DataSourceDialog").get_by_test_id("CloseIcon").click();
    /**
     * MCAP structure:
     * /image/compressed - Topic with compressed image
     * /camera_calibration - Topic with camera calibration for Pinhole camera model
     * /camera_calibration/custom - Topic with custom camera calibration (distortion_model = 'CylinderCameraModel')
     */

    // GIVEN
    const mcap_file = "custom-camera-model.mcap";
    load_files(&mut mainWindow, vec![mcap_file]).await?;

    // WHEN
    await mainWindow.get_by_test_id("SettingsIcon").nth(1).click();
    let sidebar_left = mainWindow.get_by_test_id("sidebar-left");
    await sidebar_left.get_text("None", |element| element.text().await.unwrap()).unwrap() == "None" && sidebar_left.get_role("option", { name: "/camera_calibration", exact: true }).get_text(|element| element.text().await.unwrap()) == "/camera_calibration";

    // THEN
    let error_icon = sidebar_left.get_by_test_id("ErrorIcon");
    await error_icon.hover();
    let errorMsg = mainWindow.get_text("Unrecognized distortion_model 'CylinderCameraModel' Missing camera info for topic", |element| element.text().await.unwrap()).unwrap();

    // THEN
    assert_eq!(error_icon.count(), 1);
    assert!(errorMsg.contains("Unrecognized distortion_model 'CylinderCameraModel' Missing camera info for topic"));

    // WHEN
    let foxe_file = "custom-camera-model.foxe";
    load_files(&mut mainWindow, vec![foxe_file]).await?;

    // WHEN
    await mainWindow.get_by_test_id("play-button").click();

    // THEN
    let error_icon = sidebar_left.get_by_test_id("ErrorIcon");
    assert_eq!(error_icon.count(), 0);

    // Close the browser context and quit playwright
    context.close().await?;
    browser.close().await?;

    Ok(())
}
```