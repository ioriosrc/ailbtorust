```rust
use fluentheme as ft;
use fluentuiicons::Script24Regular, Toolbox24Regular, DocumentOnePageSparkle24Regular;

use crate::components::Stack;
use crate::panels::{UserScriptEditorScript, UserScripts};
use crate::types::panels::{Script, UserScript};
use fluentui::util::make_styles;
use fluentui::theme::Theme;
use react::prelude::*;
use react::use_effect;
use react::use_state;

#[derive(Debug)]
pub struct SidebarProps {
    add_new_script: fn(script: Option<String>) -> (),
    select_script: fn(script_id: &str),
    delete_script: fn(script_id: &str),
    set_script_override: fn(script: Script, max_depth: Option<u32>),
    user_scripts: UserScripts,
    selected_script_id: Option<&str>,
    selected_script: Option<UserScript>,
    script: Option<Script>,
}

#[derive(Debug)]
pub struct Sidebar {
    scripts: UserScripts,
    selected_script_id: Option<String>,
    selected_script: Option<UserScript>,
    script_override: Option<Script>,
    active_tab: TabOption,
    tab_panels: HashMap<TabOption, Box<dyn FnOnce() -> JSXElement>>,
}

#[derive(Debug)]
pub enum TabOption {
    false, // false means no tab is selected
    "nodes",
    "utils",
    "templates",
}

fn make_styles(theme: &Theme) -> ft::StyleBundle<ft::IconProps> {
    let tab_styles = ft::icon_style({
        &mut theme.icon,
        icon_size: 24.0,
        font_size: 16.0,
        color: theme.palette.primary.main.to_color(),
    });

    ft::style_bundle!([
        ft::icon_style(tab_styles, "script-icon"),
        ft::icon_style(tab_styles, "toolbox-icon"),
        ft::icon_style(tab_styles, "document-page-icon"),
    ])
}

impl SidebarProps {
    pub fn new(
        add_new_script: fn(script: Option<String>) -> (),
        select_script: fn(script_id: &str),
        delete_script: fn(script_id: &str),
        set_script_override: fn(script: Script, max_depth: Option<u32>),
        user_scripts: UserScripts,
        selected_script_id: Option<&str>,
        selected_script: Option<UserScript>,
        script: Option<Script>,
    ) -> SidebarProps {
        SidebarProps {
            add_new_script,
            select_script,
            delete_script,
            set_script_override,
            user_scripts,
            selected_script_id,
            selected_script,
            script_override: None,
        }
    }
}

impl Sidebar {
    pub fn new(theme: &Theme) -> Sidebar {
        Sidebar {
            scripts: UserScripts::new(),
            selected_script_id: None,
            selected_script: None,
            script_override: None,
            active_tab: TabOption::false,
            tab_panels: HashMap::from([
                (TabOption::nodes, Box::new(|| <ScriptsList
                    scripts={self.scripts.clone()}
                    select_script={self.select_script}
                    delete_script={self.delete_script}
                    add_new_script={|script| self.add_new_script(script)}
                    onClose={self.close}
                    selected_script_id={self.selected_script_id.as_ref()}
                    selected_script={self.selected_script.clone()}
                    setUserScripts={self.set_user_scripts}
                />))),
                (TabOption::utils, Box::new(|| <Utilities onClose={self.close} goto_utils={self.goto_utils} script={self.script.clone()} />))),
                (TabOption::templates, Box::new(|| <Templates onClose={self.close} add_new_script={|script| self.add_new_script(script)} />))),
            ]),
        }
    }

    pub fn set_script_override(&mut self, script: Script, max_depth: Option<u32>) {
        self.script_override = Some(script);
        if let TabOption::nodes = self.active_tab {
            self.close();
        }
    }

    pub fn select_script(&mut self, script_id: &str) {
        self.selected_script_id = Some(script_id.to_string());
        self.select_script(self.scripts.get_by_id(script_id));
    }

    pub fn delete_script(&mut self, script_id: &str) {
        let script = self.user_scripts.remove(script_id);
        if let Some(script) = script {
            self.close();
        }
    }

    pub fn set_user_scripts(&mut self, scripts: UserScripts) {
        self.scripts = scripts;
        if let TabOption::nodes = self.active_tab {
            self.close();
        }
    }

    pub fn close(&mut self) {
        self.active_tab = TabOption::false;
    }

    pub fn goto_utils(&self, file_path: &str) {
        // Implement logic to handle gotoUtils
    }

    pub fn render(self) -> JSXElement {
        let theme = Theme::current();
        let styles = make_styles(theme);

        let tab_panels: HashMap<TabOption, Box<dyn FnOnce() -> JSXElement>> = self.tab_panels;
        let active_tab_style = if self.active_tab == TabOption::nodes {
            Some(styles["script-icon"])
        } else {
            None
        };

        <Paper elevation={0}>
            <Stack direction="row" full_height>
                <Tabs
                    className={styles.root}
                    orientation="vertical"
                    value={self.active_tab as i32}
                    onChange={move |event, new_value| {
                        if self.active_tab == new_value as TabOption {
                            return;
                        }
                        self.set_active_tab(new_value as TabOption);
                    }}
                >
                    <Tab
                        disableRipple
                        value="nodes"
                        title={`Scripts (${self.scripts.len()})`}
                        icon={<Script24Regular />}
                        data-testid="node-explorer"
                        onClick={if self.active_tab == "nodes" { Some(self.close) } else { None }}
                    />
                    <Tab
                        disableRipple
                        value="utils"
                        title="Utilities"
                        icon={<Toolbox24Regular />}
                        data-testid="utils-explorer"
                        onClick={if self.active_tab == "utils" { Some(self.close) } else { None }}
                    />
                    <Tab
                        disableRipple
                        value="templates"
                        title="Templates"
                        icon={<DocumentOnePageSparkle24Regular />}
                        data-testid="templates-explorer"
                        onClick={if self.active_tab == "templates" { Some(self.close) } else { None }}
                    />
                </Tabs>
                <Divider flex_item orientation="vertical" />
                <div className={self.styles.explorer_wrapper} style={active_tab_style}>
                    {tab_panels.get(&self.active_tab).unwrap()()}
                </div>
                <Divider flex_item orientation="vertical" />
            </Stack>
        </Paper>
    }
}
```