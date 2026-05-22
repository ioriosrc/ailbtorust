```rust
use crate::extension_catalog::{InstallExtensionsResult, LoadExtensionsResult};
use crate::namespace::Namespace;
use crate::{ExtensionData, ExtensionSnackbar, HttpError};
use log::error;

pub fn use InstallingExtensionsState({
    is_playing,
    player_events: { play },
}: UseInstallingExtensionsStateProps) -> UseInstallingExtensionsState {
    let install_extensions = use_extension_catalog(|state| state.install_extensions);
    const INSTALL_EXTENSIONS_BATCH = 1;

    let { set_installing_progress, start_installing_progress, reset_installing_progress } =
        use InstallingExtensionsStore(|state| ({
            set_installing_progress: state.set_installing_progress,
            start_installing_progress: state.start_installing_progress,
            reset_installing_progress: state.reset_installing_progress,
        }));
    let progress = use InstallingExtensionsStore(|state| state.installing_progress);

    let { enqueue_notification, close_notification } = use_snackbar();

    let progress_snackbar_key_ref = std::rc::Rc::new(std::cell::RefCell::new(
        `installing-extensions-${nanoid()}`,
    ));
    let progress_snackbar_key = progress_snackbar_key_ref.clone();

    // Helper function to format loader failures into user-friendly messages
    let format_failures = {
        let failed_loaders: Vec<LoadExtensionsResult & { error?: unknown }> =
            (|failed_loaders| {
                let mut messages = Vec::new();
                for failure in failed_loaders {
                    if failure.loader_type == "browser" {
                        messages.push("not saved to local cache");
                    } else if failure.loader_type == "server" {
                        messages.push("not synced to server");
                    } else {
                        messages.push(`${failure.loader_type} failed`);
                    }
                }
                messages
            })
            .borrow();

        messages.join(", ")
    };

    use_effect(move || {
        let { installed, total } = progress;
        if total == 0 || installed == total {
            close_notification(progress_snackbar_key.clone());
            return;
        }

        enqueue_notification({
            message: format!("Installing {total} extensions..."),
            variant: "info",
            persist: true,
            prevent_duplicate: true,
        });
    }, [progress, enqueue_notification, close_notification, progress_snackbar_key]);

    let install_foxe_extensions = {
        let is_playing_initial_state = is_playing;

        async move {
            start_installing_progress(total);

            let is_playing_initial_state = std::rc::Rc::downgrade(&is_playing_initial_state).unwrap();

            match install_extensions(Namespace::Local, total) {
                Ok(result) => {
                    let all_results: Vec<InstallExtensionsResult> = result
                        .into_iter()
                        .flat_map(|chunk| chunk.into_iter())
                        .collect();

                    let mut total_successful_installs = 0;
                    let mut failed_extensions: Vec<ExtensionSnackbar> = Vec::new();
                    let mut warning_extensions: Vec<ExtensionSnackbar> = Vec::new();
                    let mut cache_failures = 0;
                    let mut remote_failures = 0;

                    for result in all_results {
                        match result {
                            InstallExtensionsResult { success, loader_results } => {
                                if !success {
                                    continue;
                                }
                                total_successful_installs += 1;

                                if let Some(loader_results) = loader_results {
                                    let failed_loaders =
                                        loader_results
                                            .iter()
                                            .filter(|loader_result| !loader_result.success)
                                            .collect::<Vec<LoadExtensionsResult>>();

                                    let warning_message = format_failures(failed_loaders);

                                    warning_extensions.push({
                                        name: result.extension_name.unwrap_or("Unknown extension"),
                                        warning: `Extension installed successfully, but: ${warning_message}`,
                                        namespace,
                                    });
                                }
                            },
                            InstallExtensionsResult { error, .. } => {
                                cache_failures += 1;
                                let issue_messages = Vec::new();

                                if let Some(loader_results) = error.loader_results {
                                    let has_local_failure =
                                        loader_results
                                            .iter()
                                            .any(|loader_result| !loader_result.success);

                                    let has_remote_failure =
                                        loader_results
                                            .iter()
                                            .any(|loader_result| !loader_result.success);

                                    if has_local_failure {
                                        issue_messages.push("not saved to local cache");
                                    }

                                    if has_remote_failure && cache_failures == 0 {
                                        issue_messages.push(
                                            "Extensions were saved locally but not synced to server (offline)",
                                        );
                                    } else if has_remote_failure {
                                        issue_messages.push("Extensions were not synced to server");
                                    }
                                }

                                if !issue_messages.is_empty() {
                                    error!("Details: {}", issue_messages.join(", "));
                                }
                            },
                        };
                    }

                    set_installing_progress({
                        installed: total_successful_installs,
                    });

                    let is_playing_initial_state = is_playing_initial_state.upgrade().unwrap();

                    if !is_playing_initial_state.take().unwrap_or(true) {
                        play.unwrap();
                    }
                    reset_installing_progress();
                },
                Err(error) => {
                    error!("An error occurred during extension installation: {:?}", error);

                    log::error!("{}", error);
                },
            }
        }
    };

    return install_foxe_extensions;
}
```