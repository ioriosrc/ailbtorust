```rust
use std::error::Error;

type LayoutID = String;
type ISO8601Timestamp = String;

#[derive(Debug)]
struct RemoteLayout {
    id: LayoutID,
    name: String,
    permission: LayoutPermission,
    data: LayoutData,
    saved_at: Option<ISO8601Timestamp>,
    external_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveNewLayoutParams {
    // Define the fields of SaveNewLayoutParams here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLayoutRequest {
    // Define the fields of UpdateLayoutRequest here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLayoutResponse {
    // Define the fields of UpdateLayoutResponse here
}

pub trait IRemoteLayoutStorage {
    fn get_layouts(&self) -> Result<Vec<RemoteLayout>, Box<dyn Error>>;
    fn get_layout(&self, id: LayoutID) -> Result<Option<RemoteLayout>, Box<dyn Error>>;
    fn save_new_layout(&mut self, params: SaveNewLayoutParams) -> Result<RemoteLayout, Box<dyn Error>>;
    fn update_layout(&mut self, params: UpdateLayoutRequest) -> Result<UpdateLayoutResponse, Box<dyn Error>>;
    fn delete_layout(&mut self, id: LayoutID) -> Result<bool, Box<dyn Error>>;
}
```