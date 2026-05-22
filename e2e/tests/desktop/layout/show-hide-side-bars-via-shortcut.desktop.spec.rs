```rust
use test_env::mock::{App, AppBuilder};
use playwright::element::ElementHandle;
use playwright::error::Error as PlaywrightError;

async fn main() -> Result<(), PlaywrightError> {
    let app = AppBuilder::new()
        .build()
        .await?;

    let left_sidebar_tab_handles: Vec<ElementHandle> = app.main_frame()
        .query_selector_all(".tab")
        .await?
        .iter()
        .collect();

    for handle in &left_sidebar_tab_handles {
        handle.click().await?;
    }

    // Press '[' again to hide
    await app.main_frame().keyboard.press("[").await?;

    let right_side_variable_handles: Vec<ElementHandle> = app.main_frame()
        .query_selector_all(".variables")
        .await?
        .iter()
        .collect();

    for handle in &right_side_variable_handles {
        handle.click().await?;
    }

    // Press ']' again to hide
    await app.main_frame().keyboard.press("]").await?;

    Ok(())
}
```