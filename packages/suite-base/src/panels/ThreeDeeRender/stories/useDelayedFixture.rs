```rust
use std::sync::mpsc::Receiver;
use std::thread;

fn use_delayed_fixture(fixture_to_set: Fixture) -> Receiver<Fixture> {
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(50)); // Delay for 50 milliseconds
        tx.send(fixture_to_set).unwrap();
    });

    rx
}
```