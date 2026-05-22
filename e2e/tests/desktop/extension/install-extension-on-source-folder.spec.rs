```rust
use wry::prelude::*;

#[cfg(test)]
mod tests {
    use crate::fixtures::{electron, wait_for};

    async fn install_extension(extension_source_folder: &str) -> Result<(), wry::error::Error> {
        // When
        let main_window = electron::get_main_window().await?;

        let data_source_dialog = main_window.get_element_by_id("DataSourceDialog")?;
        await data_source_dialog.click_by_id("CloseIcon").await;

        let user_button = main_window.get_element_by_role("menuitem", Some(&"User")).await?;
        await user_button.click().await;

        let search_bar = main_window.get_element_by_placeholder("Search Extensions...").await?;
        await search_bar.fill(extension_source_folder).await;

        let turtlesim_extension = main_window
            .get_elements_by_test_id("[data-testid=\"extension-list-entry\"]")
            .await?
            .find(|el| el.text().contains("turtlesim") && el.text().contains("0.0.1"))
            .await?;

        // Then
        wait_for(|| turtlesim_extension.is_visible()).await?;

        Ok(())
    }

    #[tokio::test]
    async fn should_install_an_extension_user_folder() -> Result<(), wry::error::Error> {
        install_extension("lichtblick.suite-extension-turtlesim-0.0.1").await
    }
}
```