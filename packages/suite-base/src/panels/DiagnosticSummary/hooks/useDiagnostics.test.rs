```rust
use crate::utils::util::{compute_diagnostic_info, DiagnosticInfo};
use crate::suite_base::PanelAPI;
use crate::suite_base::panels::DiagnosticStatus::LEVELS;
use crate::suite_base::testing::builders::{
    DiagnosticsBuilder, MessageEventBuilder, BasicBuilder,
};

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_messages(diagnostics: Map<String, Map<String, DiagnosticInfo>>, messages: Vec<MessageEvent<DiagnosticStatusArrayMsg>>) -> Result<Map<String, Map<String, DiagnosticInfo>>, JsValue> {
    let mut result = diagnostics.clone();

    for message in messages {
        if let MessageEvent::Message(status_array_msg) = &message.message {
            if let Some(status) = status_array_msg.status.get(0) {
                let hardware_id = BasicBuilder.string();
                let level = BasicBuilder.sample(LEVELS);
                let diagnostic_info = DiagnosticsBuilder.info({
                    status: status,
                });
                compute_diagnostic_info(diagnostic_info);

                result.insert(
                    hardware_id,
                    result.entry(hardware_id).or_insert_with(|| {
                        Map::new()
                    })
                    .insert(status.name.clone(), diagnostic_info),
                );
            }
        }
    }

    Ok(result)
}

#[wasm_bindgen_test]
fn test_add_messages_ok_level() -> Result<(), JsValue> {
    let hardware_id = BasicBuilder.string();
    let ok_level = LEVELS.OK;
    let status_message1 = DiagnosticsBuilder.statusMessage({
        hardware_id: hardware_id,
        level: ok_level,
    });
    let message_event1 = MessageEventBuilder.messageEvent<DiagnosticStatusArrayMsg>({
        message: DiagnosticsBuilder.statusArrayMsg({
            status: [status_message1],
        }),
    });
    let info1 = DiagnosticsBuilder.info({
        status: status_message1,
    });
    compute_diagnostic_info(info1);

    let mut diagnostics = Map::new();
    diagnostics.insert(hardware_id, Map::new().insert(info1.status.name.clone(), info1));

    let result = add_messages(diagnostics, vec![message_event1]);

    assert_eq!(result.get(&hardware_id), Some(&Map::from_iter(vec![(
        "status".to_string(),
        info1,
    )])));
    Ok(())
}

#[wasm_bindgen_test]
fn test_add_messages_error_level() -> Result<(), JsValue> {
    let hardware_id = BasicBuilder.string();
    // OK LEVEL
    let ok_level = LEVELS.OK;
    let status_message1 = DiagnosticsBuilder.statusMessage({
        hardware_id: hardware_id,
        level: ok_level,
    });
    let message_event1 = MessageEventBuilder.messageEvent<DiagnosticStatusArrayMsg>({
        message: DiagnosticsBuilder.statusArrayMsg({
            status: [status_message1],
        }),
    });
    // ERROR LEVEL
    let error_level = LEVELS.ERROR;
    let status_message2 = DiagnosticsBuilder.statusMessage({
        ...status_message1,
        level: error_level,
    });
    let message_event2 = MessageEventBuilder.messageEvent<DiagnosticStatusArrayMsg>({
        message: DiagnosticsBuilder.statusArrayMsg({
            status: [status_message2],
        }),
    });

    let info1 = DiagnosticsBuilder.info({
        status: status_message1,
    });
    let info2 = DiagnosticsBuilder.info({
        status: status_message2,
    });
    compute_diagnostic_info(info2);

    let mut diagnostics = Map::new();
    diagnostics.insert(hardware_id, Map::new().insert(info1.status.name.clone(), info1));

    let result = add_messages(diagnostics, vec![message_event1, message_event2]);

    assert_eq!(result.get(&hardware_id), Some(&Map::from_iter(vec![(
        "status".to_string(),
        info2,
    )])));
    Ok(())
}

#[wasm_bindgen_test]
fn test_add_messages_no_modification() -> Result<(), JsValue> {
    let hardware_id = BasicBuilder.string();
    let status_name = BasicBuilder.string();
    let diagnostics_result: UseDiagnosticsResult = Map::new([
        [hardware_id, Map::from([(status_name, DiagnosticsBuilder.info())])],
    ]);

    let result = add_messages(diagnostics_result, []);

    assert_eq!(result, diagnostics_result);
    Ok(())
}
```