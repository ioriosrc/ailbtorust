```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// This file incorporates work covered by the following copyright and
// permission notice:
//
//   Copyright 2019-2021 Cruise LLC
//
//   This source code is licensed under the Apache License, Version 2.0,
//   found at http://www.apache.org/licenses/LICENSE-2.0
//   You may not use this file except in compliance with the License.

use std::rc::Rc;

use crate::{get_prettified_code, project_config};

use monaco_editor::{
    config::EditorConfig,
    services::{CodeEditorService, StandaloneServices},
};
use monaco_editor::{LanguageServiceFactory, LanguageServiceFactoryOptions};
use monaco_editor::{editor::ITextModel, editor::IStandaloneCodeEditor};
use react_monaco::{EditorDidMount, EditorWillMount};
use react_resized_detector::{ResizeDetector, ResizePayload};

use crate::typescript::project_config;

type CodeEditor = IStandaloneCodeEditor;

type Props = {
    script: Option<Script>;
    set_script_code: Rc<dyn Fn(String)>;
    auto_format_on_save: bool;
    ros_lib: String;
    types_lib: String;
};

fn goto_selection(editor: &CodeEditor, selection: Option<&monaco_editor::editor::ITextSelection>) {
    if let Some(selection) = selection {
        let mut selection = selection.clone();
        if !selection.is_none() && selection.end_line_number != None && selection.end_column != None {
            // These fields indicate a range was selected, set the range and reveal it.
            editor.selection(selection);
            editor.reveal_position_in_center(
                monaco_editor::Position::new(selection.start_line_number.unwrap(), selection.start_column.unwrap()),
                1,
                /* Immediate */,
            );
        } else {
            // Otherwise it's just a position
            let pos = monaco_editor::Position::new(selection.start_line_number.unwrap(), selection.start_column.unwrap());
            editor.selection(pos);
            editor.reveal_position_in_center(
                pos,
                1,
                /* Immediate */,
            );
        }
    }
}

fn get_theme() -> EditorConfig {
    if theme::is_dark_mode() {
        EditorConfig { theme: "vs-dark" }
    } else {
        EditorConfig { theme: "vs-light" }
    }
}

pub fn editor({
    auto_format_on_save,
    script,
    set_script_code,
    ros_lib,
    types_lib,
}: Props): ReactElement | ReactNull {
    let code_editor_service = StandaloneServices.get::<CodeEditorService>();
    let project_config = project_config::get_user_script_project_config();

    let editor_theme = get_theme();
    let auto_format_on_save_ref: Rc<dyn Fn(bool)> = Rc::new(move |value| {
        if value == auto_format_on_save {
            return;
        }
        auto_format_on_save_ref.store(value, std::atomic_ordering::SeqCst);
    });

    let editor_ref = Rc::new(CodeEditor::new());
    let script_override: Rc<dyn Fn(Script)> = Rc::new(move |script| {
        set_script_override(script);
    });

    use latest::use_latest;

    let save_code = move || {
        if let Some(model) = editor_ref.as_ref().get_model() && !script.is_none() && !script.read_only {
            if auto_format_on_save_ref.load(std::atomic_ordering::SeqCst) {
                model.execute_command("editor.action.formatDocument");
            }
            save(script.code());
        }
    };

    let save_code_ref = Rc::new(save_code);

    let did_mount = move |editor| {
        editor_ref.store(editor);
        editor.add_command({
            id: "ctrl-s",
            label: "Save current node",
            keybindings: vec![
                KeyMod::CtrlCmd | KeyCode::KeyS,
            ],
            run: move || {
                save_code_ref.as_ref().execute();
            },
        });
    };

    let size_ref = Rc::new(ResizeDetector::new({
        refresh_rate: 0,
        refresh_mode: RefreshMode::Debounce,
        on_resize: move |width, height| {
            if width.is_none() {
                return;
            }
            editor.as_ref().update_layout(monaco_editor::LayoutOptions { width, height });
        },
    }));

    if script.is_none() {
        // No script to load
        return Rc::new(jsx! {});
    }

    // The ErrorBoundary is required to properly capture runtime errors from Monaco Editor.
    // Without it, TypeScript or Monaco-related errors (e.g., type mismatches or input availability issues)
    // may not appear in the "Problems" tab. Do not remove this wrapper.
    return Rc::new(jsx! {
        <div ref={size_ref}>
            <ErrorBoundary>
                <Suspense fallback={<p>Loading user script editor</p>}>
                    <Editor
                        language="typescript"
                        theme={editor_theme}
                        editorWillMount={did_mount}
                        editorDidMount={did_mount}
                        options={editor_config()}
                        onChange={move |src_code| {
                            set_script_code(src_code);
                        }}
                    />
                </Suspense>
            </ErrorBoundary>
        </div>
    });
}

// Define the necessary monaco-editor modules and types here
mod typescript {
    // Define your TypeScript-related code here
}
```