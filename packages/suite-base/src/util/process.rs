```rust
use std::future::{Future, Ready};
use std::task::{Context, Poll};

struct Process {
    title: &'static str,
    browser: bool,
    env: std::collections::HashMap<String, String>,
    argv: Vec<String>,
}

impl Process {
    pub fn next_tick<F>(&self, f: F)
        where
            F: FnOnce() + 'static,
        {
            let task = Ready::new(Box::pin(f));
            futures::executor::spawn(self.run(task));
        }

    async fn run(mut self, mut task: Ready<()>):
        -> Result<(), ()> {
            while task.is_pending() {
                if self.browser {
                    self.process_next_tick(&mut task);
                } else {
                    break;
                }
            }

            Ok(())
        }

        fn process_next_tick<F>(&self, task: &mut Ready<F>)
            where
                F: FnOnce() + 'static,
            {
                let f = Box::pin(task.take().unwrap());
                futures::executor::spawn(f);
            }
        }
}

fn main() -> Result<(), ()> {
    let process = Process {
        title: "browser",
        browser: true,
        env: std::collections::HashMap::new(),
        argv: Vec::new(),
    };

    process.next_tick(|| println!("Hello from Rust!"));
    Ok(())
}
```