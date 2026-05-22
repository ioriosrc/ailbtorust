```rust
use playwright::api::{Page, MouseButton};
use playwright::{PlaywrightContext, PlaywrightOptions};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = PlaywrightContext::launch(PlaywrightOptions::default()).await?;
    let page = context.new_page().await?;

    // Given the Data Source dialog is closed
    await page.click("#DataSourceDialog > .CloseIcon", MouseButton::Left).await;

    // When the user presses clicks on the "Hide left sidebar button"
    await page.click("#left-sidebar-button", MouseButton::Left).await;

    // Then the left‐sidebar tabs are all hidden
    let left_sidebar_tabs = [
        ("panel-settings-left", false),
        ("topics-left", false),
        ("alerts-left", false),
        ("layouts-left", false),
    ];
    for (tab, expected_visible) in left_sidebar_tabs {
        if expected_visible {
            assert!(await page.locator(tab).is_visible()).await;
        } else {
            assert!(!await page.locator(tab).is_visible()).await;
        }
    }

    // When the user clicks on the "Show left sidebar" button
    await page.click("#left-sidebar-button", MouseButton::Left).await;

    // Then the left‐sidebar tabs are all visible
    for (tab, expected_visible) in left_sidebar_tabs {
        if expected_visible {
            assert!(await page.locator(tab).is_visible()).await;
        } else {
            assert!(!await page.locator(tab).is_visible()).await;
        }
    }

    // Given the Data Source dialog is closed
    await page.click("#DataSourceDialog > .CloseIcon", MouseButton::Left).await;

    // When the user presses clicks on the "Show right sidebar button"
    await page.click("#right-sidebar-button", MouseButton::Left).await;

    // Then the right‐sidebar tabs are all visible
    assert!(await page.locator("variables-right").is_visible()).await;

    // When the user clicks on the "Hide right sidebar" button
    await page.click("#right-sidebar-button", MouseButton::Left).await;

    // Then the right‐sidebar tabs are all hidden
    assert!(!await page.locator("variables-right").is_visible()).await;

    Ok(())
}
```

Este código Rust utiliza a Playwright para simular interações do navegador. Ele abre uma nova página em uma instância de Playwright, navega até um elemento com a ID "DataSourceDialog", clicka no botão "CloseIcon" e espera o diálogo ser fechado. Em seguida, ele pressiona o botão "Hide left sidebar button" e verifica que todos os itens do lado esquerdo são invisíveis. Depois disso, o botão é clicado novamente para tornar os itens visíveis de volta. A mesma sequência é realizada para o lado direito, mas com um botão diferente e um elemento diferente no lado direito.