```rust
use std::time::{Duration, SystemTime};

use lighthouse_core::{
    context::CurrentLayoutContext,
    services::{ILayoutStorage, IRemoteLayoutStorage},
};
use lighthouse_test_builders::layout::*;

#[tokio::test]
async fn update_or_fetch_layout() {
    let mut mock_remote_storage: Box<dyn IRemoteLayoutStorage> = Box::new(MockRemoteLayoutStorage {
        workspace: String::from("default"),
        get_layouts: MockedMethod::new(|_| Ok(vec![])),
        get_layout: MockedMethod::new(|_| Err(SystemTime::now() + Duration::from_secs(1))),
        save_new_layout: MockedMethod::new(|_| Ok(())),
        update_layout: MockedMethod::new(|_| {
            Ok(RemoteLayoutBuilder::default().id(BasicString::from("test_id")).build())
        }),
        delete_layout: MockedMethod::new(|_| Ok(())),
    });

    let update_params = BasicRemoteLayout::builder()
        .id(BasicString::from("test_id"))
        .name(BasicString::from("test_name"))
        .saved_at(SystemTime::now().to_string())
        .build();

    let mut result = update_or_fetch_layout(&mut mock_remote_storage, update_params).await.unwrap();
    assert_eq!(result.id(), BasicString::from("test_id"));

    mock_remote_storage.update_layout
        .mock_with(|_| Err(SystemTime::now() + Duration::from_secs(1)));
    result = update_or_fetch_layout(&mut mock_remote_storage, update_params).await.unwrap();
    assert_eq!(result.id(), BasicString::from("test_id"));

    mock_remote_storage.get_layout
        .mock_with(|_| Ok(RemoteLayoutBuilder::default().id(BasicString::from("nonexistent")).build()));
    let error = update_or_fetch_layout(&mut mock_remote_storage, update_params).await.unwrap_err();
    assert_eq!(error.to_string(), "Update rejected but layout is not present on server: test_id");
}
```