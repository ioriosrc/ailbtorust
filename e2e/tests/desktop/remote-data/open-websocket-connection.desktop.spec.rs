```rust
use test::{test, expect};
use electron::fixtures::launch_websocket;

#[test]
async fn show_correctly_open_a_web_socket_connection_showing_correct_attributes_on_raw_messages_panel() {
    // Given
    let websocket_server = launch_websocket();

    // When
    await mainWindow.get_text("Open connection").click();
    await mainWindow.get_text("Open", { exact: true }).click();

    // Then
    await expect(mainWindow.get_text("ws://localhost:8765").inner_html()).resolves_to(None);

    // When
    await mainWindow.get_text("Topics", { exact: true }).click();
    await mainWindow.get_text("/websocket_test").inner_html().resolves_to(None);
    await mainWindow.get_data-testid("AddPanelButton").click();
    await mainWindow.get_text("Raw Messages", { exact: true }).click();
    await mainWindow.get_placeholder("/some/topic.msgs[0].field").nth(0).click();
    await mainWindow.get_data-testid("autocomplete-item").click();

    let raw_messages_panel = mainWindow.get_data-testid(/RawMessages/);
    await raw_messages_panel.get_text("data").nth(0).click();
    let attributes_to_check = ["hello", '"world"', "foo", "42"];

    for attribute in attributes_to_check {
        await expect(raw_messages_panel.get_text(attribute, { exact: true }).text()).resolves_to(Some(attribute));
    }

    void websocket_server.close();
}
```