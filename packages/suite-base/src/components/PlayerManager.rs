```rust
use anyhow::{anyhow, Result};
use async_std::sync::Arc;
use async_std::task;
use async_store::{AsyncIndexDbRecents, IndexedDbError};

async fn try_get_file(file_path: &str) -> std::io::Result<fs::File> {
    fs::File::open(file_path).await.map_err(|err| anyhow!("Failed to open file: {}", err))
}

fn merge_multiple_file_names(handles: &[std::path::PathBuf]) -> String {
    handles.iter().map(|h| h.to_string_lossy()).collect::<Vec<String>>()
        .join(", ")
}

#[derive(Clone)]
struct UserScriptPlayer {
    topic_alias_player: TopicAliasingPlayer,
    user_script_actions: Vec<UserScriptAction>,
    metrics_registry: Arc<PerformanceMetricsCollector>,
}

impl UserScriptPlayer {
    pub fn new(
        topic_alias_player: TopicAliasingPlayer,
        user_script_actions: Vec<UserScriptAction>,
        metrics_registry: Arc<PerformanceMetricsCollector>,
    ) -> Self {
        Self {
            topic_alias_player,
            user_script_actions,
            metrics_registry,
        }
    }

    pub async fn set_global_variables(&self, global_variables: GlobalVariables) {
        self.topic_alias_player.set_global_variables(global_variables);
    }

    pub async fn initialize(
        &self,
        file: Option<fs::File>,
        files: Option<Vec<fs::File>>,
        metrics_registry: Arc<PerformanceMetricsCollector>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut player = self.topic_alias_player.clone();

        if let Some(file) = file {
            player.initialize_with_file(file);
        } else if let Some(files) = files {
            player.initialize_with_files(files);
        }

        Ok(player)
    }
}

struct TopicAliasingPlayer {
    base_player: Player,
}

impl TopicAliasingPlayer {
    pub fn new(base_player: Player) -> Self {
        Self { base_player }
    }

    pub fn set_alias_functions(&self, alias_functions: Immutable<TopicAliasFunctions>) {
        self.base_player.set_alias_functions(alias_functions);
    }
}

struct AnalyticsMetricsCollector {
    analytics: Arc<AnalyticsContext>,
}

impl AnalyticsMetricsCollector {
    pub fn new(analytics: Arc<AnalyticsContext>) -> Self {
        Self { analytics }
    }

    pub fn set_property(&self, key: &str, value: String) {
        self.analytics.set_property(key, value);
    }
}

struct Player {
    // Define player properties and methods
}

#[derive(Clone)]
struct UserScriptAction {
    // Define user script action properties and methods
}

impl PlayerManagerProps {
    pub fn new(player_sources: Vec<IDataSourceFactory>) -> Self {
        Self { player_sources }
    }
}

const EMPTY_USER_NODES: GlobalVariables = GlobalVariables::new();
const EMPTY_GLOBAL_VARIABLES: GlobalVariables = GlobalVariables::new();

async fn select_source(
    player_sources: &Vec<IDataSourceFactory>,
    args: DataSourceArgs,
) -> Result<Player, Box<dyn std::error::Error>> {
    let found_source = player_sources
        .iter()
        .find(|source| source.id == args.source_id || source.legacy_ids.contains(&args.source_id))
        .ok_or(anyhow!("Unknown data source: {}", args.source_id))?;

    // Perform metrics collection
    MetricsRegistry::set_property("player", found_source.id);

    let base_player = found_source.initialize_with_args(args)?;

    Ok(base_player)
}

async fn select_recent(
    recents: &[RecentRecord],
    select_source: &dyn Fn(&str, DataSourceArgs) -> Result<Player, Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for recent in recents {
        match recent.type_ {
            "connection" => {
                let args = DataSourceArgs {
                    type_: "connection",
                    params: recent.extra,
                };
                select_source(&recent.source_id, args)?;
            }
            "file" => {
                let args = DataSourceArgs {
                    type_: "file",
                    handles: recent.handles.clone(),
                };
                select_source(&recent.source_id, args)?;
            }
        }
    }

    Ok(())
}

async fn create_select_recent_callback(
    recents: &[RecentRecord],
    select_source: &dyn Fn(&str, DataSourceArgs) -> Result<Player, Box<dyn std::error::Error>>,
    enqueue_snackbar: &mut impl std::io::Write,
) -> Box<dyn Fn(&str)> {
    Box::new(move |recent_id| {
        if let Some(found_recent) = recents.iter().find(|value| value.id == recent_id) {
            match found_recent.type_ {
                "connection" => {
                    select_source(&found_recent.source_id, DataSourceArgs {
                        type_: "connection",
                        params: found_recent.extra,
                    })?;
                }
                "file" => {
                    select_source(&found_recent.source_id, DataSourceArgs {
                        type_: "file",
                        handles: found_recent.handles.clone(),
                    })?;
                }
            }

            if let Err(err) = enqueue_snackbar.write_all(format!("Failed to restore recent: {}", err).as_bytes())? {
                eprintln!("Error writing to snackbar: {:?}", err);
            }
        } else {
            enqueue_snackbar.write_all("Failed to restore recent".as_bytes())?;
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize context and other dependencies here

    let recents = async_std::task::block_on(async move {
        AsyncIndexDbRecents::get_recent_entries().await
    })?;

    let select_source = |id, args| task::block_on(select_source(&recents, args));

    let enqueue_snackbar = &mut std::io::stdout();

    let create_select_recent_callback_result = create_select_recent_callback(
        &recents,
        &select_source,
        enqueue_snackbar,
    );

    let value: PlayerSelection = {
        select_source,
        select_recent: create_select_recent_callback_result,
        selected_source: None,
        available_sources: recents.clone(),
        recent_sources: Vec::new(),
    };

    Ok(())
}
```