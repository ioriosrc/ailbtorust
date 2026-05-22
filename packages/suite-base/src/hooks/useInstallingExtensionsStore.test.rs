```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let state = Arc::new(Mutex::new(InstallationState {
        installed: 0,
        total: 0,
        in_progress: false,
    }));

    thread::spawn(move || {
        act(|| {
            use InstallingExtensionsStore::start_installation_progress(&state, BasicBuilder.number({ min: 0, max: 150 }).value());
        });

        let mut state = state.lock().unwrap();
        assert_eq!(state.installing_progress(), &InstallationProgress {
            installed: 0,
            total: BasicBuilder.number({ min: 0, max: 150 }).value(),
            in_progress: true,
        });
    });

    thread::spawn(move || {
        act(|| {
            use InstallingExtensionsStore::set_installation_progress(&state, BasicBuilder.number({ min: 0, max: 150 }).value());
        });

        let mut state = state.lock().unwrap();
        assert_eq!(state.installing_progress(), &InstallationProgress {
            installed: BasicBuilder.number({ min: 0, max: 150 }).value(),
            total: BasicBuilder.number({ min: 0, max: 150 }).value(),
            in_progress: true,
        });
    });

    thread::spawn(move || {
        act(|| {
            use InstallingExtensionsStore::reset_installation_progress(&state);
        });

        let mut state = state.lock().unwrap();
        assert_eq!(state.installing_progress(), &InstallationProgress {
            installed: 0,
            total: 0,
            in_progress: false,
        });
    });
}

struct InstallationState {
    installed: u32,
    total: u32,
    in_progress: bool,
}

trait InstallingExtensionsStore {
    fn start_installation_progress(&self, extensions_number: u32);
    fn set_installation_progress(&self, new_installed: u32);
    fn reset_installation_progress(&self);
}

struct MockInstallingExtensionsStore {
    installed: Arc<Mutex<u32>>,
    total: Arc<Mutex<u32>>,
    in_progress: bool,
}

impl InstallingExtensionsStore for MockInstallingExtensionsStore {
    fn start_installation_progress(&self, extensions_number: u32) {
        let mut state = self.installed.lock().unwrap();
        *state += extensions_number;
        self.total.lock().unwrap() += extensions_number;
        self.in_progress = true;
    }

    fn set_installation_progress(&self, new_installed: u32) {
        let mut state = self.installed.lock().unwrap();
        *state = new_installed;
        self.total.lock().unwrap() = new_installed;
        self.in_progress = true;
    }

    fn reset_installation_progress(&self) {
        let mut state = self.installed.lock().unwrap();
        *state = 0;
        self.total.lock().unwrap() = 0;
        self.in_progress = false;
    }
}

#[derive(Debug)]
struct InstallationProgress {
    installed: u32,
    total: u32,
    in_progress: bool,
}
```