```rust
use wasm_bindgen::prelude::*;
use js_sys::Array;
use web_sys::{Document, Element};

#[wasm_bindgen]
pub async fn create_new_layout_with_raw_messages_panel_and_select_topic_and_change_font_size() {
    let document = doc!();
    let layout_list_item = document.get_element_by_id("layout-list-item");
    let panel_search = document.get_element_by_id("panel-list-textfield");

    // Given
    load_files();

    // When
    layout_list_item.click().unwrap();
    let create_new_layout_button = document.get_element_by_id("create-new-layout").unwrap();
    create_new_layout_button.click().unwrap();
    panel_search.fill("Raw Messages");
    panel_search.dispatch_event(&Event::new("input")).unwrap();
    let raw_messages_panel = document.get_elements_by_class_name("raw-messages-panel").nth(0).unwrap();

    // Then
    let play_button = raw_messages_panel.get_element_by_role("button", Some(&JsValue::from_str("Play")));
    assert!(play_button.is_some());
    assert_eq!(play_button.unwrap().get_attribute("disabled"), "false");
    let no_topic_selected_text = raw_messages_panel.get_text_content();
    assert!(no_topic_selected_text.contains("No topic selected"));

    // When
    let topic_path_input = raw_messages_panel.get_element_by_placeholder("/some/topic.msgs[0].field", Some(&JsValue::from_str("/rosout")));
    topic_path_input.dispatch_event(&Event::new("input")).unwrap();
    let no_topic_selected_text = raw_messages_panel.get_text_content();
    assert!(!no_topic_selected_text.contains("No topic selected"));
    let topic_message = raw_messages_panel.get_element_by_text("level 1");
    assert!(topic_message.is_some());
    let font_size_css_value = topic_message.unwrap().get_attribute("style").unwrap().split(';').nth(0).unwrap();
    assert_eq!(font_size_css_value.contains("font-size"), true);
    assert_eq!(font_size_css_value.split(' ').collect::<Vec<&str>>()[1], "12px");

    // When
    let panel_settings_left = raw_messages_panel.get_element_by_id("panel-settings-left");
    panel_settings_left.click().unwrap();
    let field_editor_select_button = document.get_element_by_id("FieldEditor-Select").unwrap();
    field_editor_select_button.click().unwrap();
    topic_message.dispatch_event(&Event::new("input")).unwrap();
    topic_message.get_attribute("style").unwrap().split(';').nth(1).unwrap();
    assert_eq!(topic_message.unwrap().get_attribute("style").unwrap().split(' ').collect::<Vec<&str>>()[1], "30px");
}
```

Este código Rust usa `wasm-bindgen` para interagir com o DOM do navegador, simulando o comportamento de uma aplicação web. Ele carrega um arquivo `.mcap`, cria um novo layout, seleciona a rota `/rosout`, altera o tamanho da fonte dos messagens e verifica se essas mudanças foram aplicadas corretamente.