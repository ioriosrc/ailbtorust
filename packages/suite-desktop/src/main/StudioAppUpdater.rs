```rust
use std::thread;

#[derive(Debug)]
struct UpdateInfo {
    version: String,
}

struct Logger;

impl Logger {
    fn getLogger(_filename: &str) -> Self {
        Self {}
    }
}

fn is_network_error(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::ConnectionRefused ||
        err.kind() == std::io::ErrorKind::BrokenPipe ||
        err.kind() == std::io::ErrorKind::NotConnected ||
        err.kind() == std::io::ErrorKind::TimedOut
}

struct AppSetting {
    is_updates_enabled: bool,
}

impl AppSetting {
    fn get_setting(&self, setting_name: &str) -> Option<bool> {
        // Simulate getting the app setting from a database or other storage
        if setting_name == "UPDATES_ENABLED" && self.is_updates_enabled {
            Some(true)
        } else {
            None
        }
    }
}

struct StudioAppUpdater;

impl StudioAppUpdater {
    fn new() -> Self {
        Self {}
    }

    fn start(&mut self) {
        let mut timeout = std::time::Duration::from_secs(60 * 10);
        while !self.can_check_for_updates() {
            thread::sleep(timeout);
            timeout *= 2;
        }
        log::info!("Starting update loop");
        std::thread::spawn(move || {
            loop {
                self.check_now();
                let delay = std::time::Duration::from_secs(60 * 60);
                log::info!("Waiting for new check");
                thread::sleep(delay);
            }
        });
    }

    fn can_check_for_updates(&self) -> bool {
        // Updates are disabled by default in dev mode
        true
    }

    async fn check_now(&mut self) {
        let on_disabled = || {
            log::info!("Updates are not enabled.");
        };
        let on_not_available = |info: UpdateInfo| {
            log::info!("Lichtblick is up to date (version {}).", info.version);
        };
        let onError = |error: Box<dyn std::error::Error>| {
            log::error!("{}", error);
            dialog::alert("An error occurred while checking for updates.");
        };

        if !self.can_check_for_updates() {
            on_disabled();
            return;
        }
        match auto_updater().check_for_updates_and_notify() {
            Ok(result) => {
                if result {
                    on_not_available(auto_updater().get_current_version());
                } else {
                    log::info!("No updates available.");
                }
            },
            Err(error) => {
                onError(Box::new(error));
            }
        };
    }

    fn #maybe_check_for_updates(&mut self) {
        match auto_updater().check_for_updates_and_notify() {
            Ok(result) => {
                if result {
                    log::info!("Checking for updates");
                    std::thread::sleep(std::time::Duration::from_secs(60 * 10));
                    self.#maybe_check_for_updates();
                }
            },
            Err(error) => {
                if is_network_error(&error) {
                    log::warn!("Network error checking for updates: {}", error);
                } else {
                    self.emit("error", error.clone());
                }
            }
        };
    }

    fn instance() -> Self {
        static mut INSTANCE: Option<StudioAppUpdater> = None;
        INSTANCE.get_or_insert_with(|| StudioAppUpdater {})
    }
}

fn main() {
    let app_updater = StudioAppUpdater::instance();
    app_updater.start();
}
```