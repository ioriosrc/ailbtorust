```rust
use std::sync::{Arc, CondvarLock};
use std::thread;

struct Condvar {
    lock: CondvarLock<()>,
    notify_channel: Arc<(std::sync::mpsc::Sender<()> + Send)>,
    wait_queue: Vec<Box<dyn FnMut()>>,
}

impl Condvar {
    fn new() -> Self {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel();
        Self {
            lock: CondvarLock(()),
            notify_channel,
            wait_queue: vec![],
        }
    }

    async fn wait(&self) {
        let mut guard = self.lock.lock().await;
        self.wait_queue.push(Box::new(move || notify_tx.send(()).unwrap()));
        drop(guard);
        while !self.notify_rx.try_recv().is_ok() {}
    }

    fn notify_one(&self) {
        let _ = self.notify_channel.send(()).unwrap();
    }

    fn notify_all(&self) {
        for &item in self.wait_queue.iter_mut() {
            (*item)();
        }
        self.wait_queue.clear();
    }
}

fn main() {
    let condvar = Condvar::new();

    thread::spawn(move || {
        println!("Producer is waiting...");
        condvar.wait().unwrap();
        println!("Producer notified!");
    });

    thread::spawn(move || {
        println!("Consumer is notified and going to notify the producer...");
        condvar.notify_one();
        println!("Producer is again notified!");
    });
}
```