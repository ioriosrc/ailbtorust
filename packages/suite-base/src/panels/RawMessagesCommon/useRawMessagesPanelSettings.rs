```rust
use react_hooks::use_callback;
use react_hooks::use_effect;
use react_hooks::use_state;

type UseRawMessagesPanelSettingsOptions = {
  fontSize: Option<f64>;
  save_config: fn(&mut Options);
};

fn use_raw_messages_panel_settings({ fontSize, save_config }: UseRawMessagesPanelSettingsOptions) -> () {
    let update_panel_settings_tree = use_context::<ProviderContext>());
    let action_handler = use_callback(
        move |action| {
            if (
                action.action == "update" &&
                action.payload.path[0] == "general" &&
                action.payload.path[1] == "fontSize"
            ) {
                save_config(&mut Options {
                    fontSize: action.payload.value.map(|v| v as f64),
                });
                return;
            }
        },
        [save_config],
    );

    use_effect(move || {
        update_panel_settings_tree({
            action_handler,
            nodes: {
                general: {
                    label: "General",
                    fields: {
                        fontSize: {
                            label: "Font size",
                            input: "select",
                            options: [
                                { label: "auto", value: None },
                                ...FONT_SIZE_OPTIONS.map(|value| ({
                                    label: format!("{} px", value),
                                    value: Some(value as f64),
                                })),
                            ],
                            value: fontSize,
                        },
                    },
                },
            },
        });
    }, [action_handler, fontSize, update_panel_settings_tree]);
}
```