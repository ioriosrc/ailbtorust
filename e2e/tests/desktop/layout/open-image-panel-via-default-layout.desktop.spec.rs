```rust
use playwright::playwright_core::{Browser, BrowserContext, Page};

async fn test_images_panel_settings(browser_context: &BrowserContext) -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let page = browser_context.new_page()?;
    await page.goto("http://example.com")?;

    // WHEN the user clicks on the Images panel
    await page.click("#DataSourceDialog > .CloseIcon", "#layouts-left");
    await page.click("#layout-list-item:has-text('Default')");

    // THEN the Images panel settings should be displayed
    await page.wait_for_selector("#panel-settings-left button:has-text('Image')");
    let count = await page.query_selector_all("div[data-testid='State Transitions panel']").count();
    assert_eq!(count, 1);

    Ok(())
}
```

O código acima é uma função Rust que realiza a mesma tarefa do test anterior. Ele abre um navegador, navega até uma página de exemplo, clica em um botão para fechar uma janela e em outra para abrir o painel de layouts. Em seguida, ele clica no painel de layout padrão e verifica se o botão "Image" existe no painel de configurações do estado transição.