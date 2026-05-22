```rust
use crate::components::{ScriptListItem, SidebarHeader};
use crate::types::{Scripts, UserScript};
use styled_components::{create_container, ThemeProvider};

type ScriptsListProps = {
    scripts: Scripts;
    add_new_script: fn();
    select_script: fn(script_id: &str);
    delete_script: fn(script_id: &str);
    on_close: fn();
    selected_script_id: Option<&str>;
    selected_script: Option<UserScript>;
    set_user_scripts: fn(scripts: Scripts);
};

#[create_container("scripts_list")]
pub fn ScriptsList(props: ScriptsListProps) -> styled_components::Component {
    let { classes, ...rest } = props;

    styled_components::div {
        flex: "auto";
        display: "flex";
        justify-content: space-between";
        align-items: center;
    }
}

impl ScriptsList {
    pub fn render(&self) -> StyledComponents::ElementRef {
        let { scripts, add_new_script, select_script, delete_script, on_close, selected_script_id, selected_script, set_user_scripts } = self;

        styled_components::div {
            flex: "auto";
            display: "flex";
            justify-content: space-between";
            align-items: center;
        }

        <SidebarHeader title="Scripts" onClose={on_close} />
        <List>
            {scripts
                .iter()
                .enumerate()
                .map(|(script_id, script)| {
                    ScriptListItem {
                        key: script_id.to_string(),
                        title: match &script.name {
                            Some(name) => name.clone(),
                            None => "Untitled script".to_string(),
                        },
                        selected: selected_script_id == Some(&script_id),
                        onClick: move || select_script(&script_id),
                        onDelete: move || delete_script(&script_id),
                        on_rename: |name| {
                            if let Some(selected_script) = &selected_script {
                                set_user_scripts({
                                    ..scripts,
                                    [script_id]: { ..selected_script, name },
                                });
                            }
                        },
                    }
                })
                .collect::<Vec<StyledComponents::ElementRef>>()}
            <li class={classes.buttonRow}>
                <Button
                    fullWidth
                    startIcon={<AddIcon />}
                    variant="contained"
                    color="inherit"
                    onClick={add_new_script}
                >
                    New script
                </Button>
            </li>
        </List>
    }
}
```