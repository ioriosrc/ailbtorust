```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use crate::components::{Config, Script};
use crate::provider::PanelStateContextProvider;
use crate::store::UserScriptStore;
use crate::types::{LayoutState, UserScripts};

use anyhow::{Error, Result};
use axum::{
    extract::{Json, Query},
    routing::{get, post},
    Router,
};
use chrono::Local;
use log::{debug, info};
use serde_json::{json, to_string_pretty};
use std::time::Duration;

pub async fn user_script_editor(
    config: Json<Config>,
    mut user_scripts_store: UserScriptStore,
    layout_state: Query<LayoutState>,
) -> Result<Json<UserScripts>, Error> {
    let selected_node_id = config.selected_node_id;
    let auto_format_on_save = config.auto_format_on_save;

    // Your script can output well-known message types, any of your custom message types, or
    // complete custom message types.
    //
    // Use \`Message\` to access types from the schemas defined in your data source:
    // type Twist = Message<"geometry_msgs/Twist">;
    //
    // Import from the @foxglove/schemas package to use foxglove schema types:
    // import { Pose, LocationFix } from "@foxglove/schemas";
    //
    // Conventionally, it's common to make a _type alias_ for your script's output type
    // and use that type name as the return type for your script function.
    // Here we've called the type \`Output\` but you can pick any type name.
    // type Output = {
    //   hello: string;
    // };

    // These are the topics your script "subscribes" to. Studio will invoke your script function
    // when any message is received on one of these topics.
    let inputs = vec!["/input/topic"];

    // Any output your script produces is "published" to this topic. Published messages are only visible within Studio, not to your original data source.
    let output = "/studio_script/output_topic";

    // This function is called with messages from your input topics.
    // The first argument is an event with the topic, receive time, and message.
    // Use the \`Input<...>\` helper to get the correct event type for your input topics.
    async fn handle_input(event: serde_json::Value) -> Result<(), Error> {
        debug!("Received input: {:?}", event);
        Ok(())
    }

    // Setup routes for the user script editor
    let app = Router::new()
        .route("/", get(user_script_editor))
        .route("/inputs", post(handle_input));

    // Run the server
    info!("Starting server on port 3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_state(layout_state))
        .await?;

    Ok(Json(user_scripts_store.read()))
}
```