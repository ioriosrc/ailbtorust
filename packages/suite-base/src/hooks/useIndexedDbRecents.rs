```rust
use std::collections::HashMap;

type RecentRecordCommon = {
    id: String;

    source_id: String;

    title: String;

    label: Option<String>;
};

#[derive(Clone, Debug)]
struct RecentConnectionRecord {
    type: &'static str;

    extra: HashMap<&'static str, Option<serde_json::Value>>;
}

#[derive(Clone, Debug)]
struct RecentFileRecord {
    handles: Vec<FileHandle>,
}

impl RecentRecordCommon for RecentConnectionRecord {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn source_id(&self) -> &str {
        self.source_id.as_str()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl RecentRecordCommon for RecentFileRecord {
    fn id(&self) -> &str {
        "file"
    }

    fn source_id(&self) -> &str {
        self.handles.first().map(|handle| handle.id()).unwrap_or("")
    }

    fn title(&self) -> &str {
        // Assuming FileHandle::name() is available
        self.handles.first().map(|handle| handle.name()).unwrap_or("")
    }
}

type OldRecentRecord = {
    handle: Option<FileHandle>,
};

#[derive(Clone, Debug)]
struct UnsavedRecentRecord {
    type: &'static str;

    extra: HashMap<&'static str, Option<serde_json::Value>>;
}

impl UnsavedRecentRecord for RecentConnectionRecord {
    fn source_id(&self) -> &str {
        self.source_id.as_str()
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl UnsavedRecentRecord for RecentFileRecord {
    fn handles(&self) -> &[FileHandle] {
        &self.handles
    }
}

type IRecentsStore = {
    recents: Vec<RecentRecord>;

    add_recent: fn(&mut Self, record: UnsavedRecentRecord);

    save: fn(&mut Self);
};

fn useIndexedDbRecents() -> IRecentsStore {
    let mut db = Database::open("lichtblick-recents", "1.0");

    async move {
        let mut recents: Vec<RecentRecord> = db.get(Key::FromStr(IDB_KEY).unwrap()).await.unwrap_or_default();

        if !db.is_open() {
            return IRecentsStore {
                recents,
                add_recent,
                save,
            };
        }

        let new_recents = db
            .write()
            .map_err(|e| log::error(e))
            .await?;

        IRecentsStore {
            recents,
            add_recent,
            save,
        }
    }?
}

fn main() {
    // Implementation of the main function goes here
}
```