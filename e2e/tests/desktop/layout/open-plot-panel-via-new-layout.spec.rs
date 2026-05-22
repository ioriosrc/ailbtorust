```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

mod tests {
    use async_std::io::{self, BufReader, Write};
    use serde_json::{Map, Value};

    async fn load_files(main_window: &web_sys::HtmlDocument) -> Result<(), String> {
        // Given a file is loaded and a new layout is created with a Plot panel
        let filename = "example-2.mcap";
        main_window
            .document()
            .get_element_by_id("layouts-left")
            .unwrap()
            .click();
        main_window
            .document()
            .get_element_by_id("create-new-layout")
            .unwrap()
            .click();

        // When
        // the user opens a Plot panel and adds a series with "mouse.clientX"
        main_window
            .document()
            .get_element_by_id("panel-settings-left")
            .unwrap()
            .click();
        main_window
            .document()
            .get_text_content::<web_sys::Node>()
            .await?
            .contains("Plot")?;

        await mainWindow.get_element_by_text("/some/topic.msgs[0].field").unwrap().click();

        // Then
        // "mouse.clientX" should appear in the Plot panel as the path of the added series
        let plot_legend_row_path_label = main_window.document().get_element_by_id("plot-legend-row-path-label").unwrap();
        assert!(plot_legend_row_path_label.has_text_content(), "{}", plot_legend_row_path_label.text_content());

        Ok(())
    }
}
```