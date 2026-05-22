```rust
use crate::components::{Editor, Script};
use crate::config::{DEFAULT_STUDIO_SCRIPT_PREFIX, user_script_project_config};
use crate::mocks::{
    create_mock_editor, format_action, mock_monaco_api, mock_open_handler,
    set_script_code, set_script_override, test_user_script_player_with_formatting_errors,
};
use crate::utils::{BasicBuilder, resize_detector_options};

#[test]
fn auto_format_on_save() {
    let mut base_script = Script::new(BasicBuilder.string(), BasicBuilder.string());
    base_script.read_only = false;

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &base_script,
        set_script_code: |_| {},
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, editor| {
            assert_eq!(input.resource.path, base_script.filePath);
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, BasicBuilder.string());
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, BasicBuilder.string());
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.expect().with_args(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn read_only_script() {
    let mut base_script = Script::new(BasicBuilder.string(), BasicBuilder.string());
    base_script.read_only = true;

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &base_script,
        set_script_code: |_| {},
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, base_script.filePath);
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, base_script.filePath);
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, base_script.filePath);
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, base_script.code);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.expect().with_args(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});
    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn jump_inside_the_current_script() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    resize_detector.expect().with_args(&editor, Some(0), None);

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    resize_detector.expect().with_args(&editor, Some(0), None);

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            set_script_override.expect_with(|_| {});
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, `interface some-type {}`);

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    resize_detector.expect().with_args(&editor, Some(0), None);

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let mut editor = create_mock_editor();

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn request_to_open_another_model() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn position_only_selection() {
    let mut set_script_override = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        set_script_override: &set_script_override,
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///node_modules/@types/some-type.d.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn unknown_model() {
    let rendered_editor = render::<Editor>(None);

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///unknown/model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn stale_model() {
    let mut set_script_code = Box::new(|| {});

    let rendered_editor = render::<Editor>(Some({
        auto_format_on_save: true,
        script: &Script::new(BasicBuilder.string(), BasicBuilder.string()),
        set_script_code: &set_script_code,
        save: |_| {},
        set_script_override: |_| {},
        ros_lib: BasicBuilder.string(),
        types_lib: BasicBuilder.string(),
    }));

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    mock_open_handler.expect().with_args(
        |input, _| {
            assert_eq!(input.resource.path, "file:///stale-model.ts");
            let model = editor.get_model(input.resource.uri).unwrap();
            assert_eq!(model.value, BasicBuilder.string());

            format_action.run_with(|_| async move { Ok(()) });
        },
    );

    format_action.run_with(|_| async move { Ok(()) });

    test_user_script_player_with_formatting_errors(&mut rendered_editor);
}

#[test]
fn resize_without_width() {
    let