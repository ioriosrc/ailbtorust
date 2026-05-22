```rust
use material_ui_core::{Stack, ListItem, ListItemButton, ListItemText};
use suite_base::components::Stack as StackComponent;
use suite_base::panels::UserScriptEditor::script::Script;
use suite_base::players::UserScriptPlayer::transformer_worker::typescript::project_config::{utility_files, getUserScriptProjectConfig};

fn Utilities({
    onClose,
    goto_utils,
    script: Option<&Script>,
}: {
    onClose: fn(),
    goto_utils: fn(String),
    script: Option<&Script>,
}) -> impl 'static + ReactNode {
    StackComponent {
        flex: "auto",
        position: Position::Relative,
        children: vec![
            SidebarHeader::new(onClose, "Utilities", Some(
                utility_files().iter()
                    .map(|file| format!("import {{ ... }} from \"./{}.ts\".", file.fileName))
                    .collect::<Vec<String>>())
                    .unwrap_or_default(),
            )),
            ListComponent {
                dense: true,
                children: vec![
                    utility_files().iter()
                        .map(|file| {
                            ListItemComponent::new(
                                false,
                                Some(file.filePath),
                                Some(ListItemButtonComponent {
                                    selected: script.map(|s| s.filePath == file.filePath).unwrap_or_default(),
                                    children: vec![ListItemTextComponent {
                                        primary: TextComponent::new(format!("{}.ts", file.fileName)),
                                        slotProps: {
                                            primary: TextComponentProps { variant: "body2" },
                                        },
                                    }],
                                }),
                            )
                        })
                        .collect::<Vec<ReactNode>>()),
                    ListItemButtonComponent {
                        selected: script.map(|s| s.filePath == "/studio_script/generatedTypes.ts").unwrap_or_default(),
                        children: vec![ListItemTextComponent {
                            primary: TextComponent::new("/studio_script/generatedTypes.ts"),
                        }],
                    },
                ],
            },
        ],
    }
}
```