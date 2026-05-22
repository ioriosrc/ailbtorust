```rust
use std::process::{Command, Stdio};
use async_std::task;

async fn exec_output(program: &str, args: Option<&[&str]>, options: Option<ExecOptions>) -> Result<(i32, String), Box<dyn std::error::Error>> {
    let mut child = Command::new(program);
    if let Some(args) = args {
        for arg in args {
            child.arg(arg);
        }
    }

    let output = child
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    let mut stdout = String::new();
    let mut stderr = String::new();

    if let Err(e) = async_std::task::block_on(async move {
        let mut stdin = output.lock().unwrap().take();
        let mut stdout = stdout.lock().unwrap().take();
        let mut stderr = stderr.lock().unwrap().take();

        while let Some(line) = stdin.next() {
            stdout.write_all(&line?).await?;
        }

        if !stderr.is_empty() {
            error!("Error: {}", stderr);
        }
    }) {
        return Err(e.into());
    }

    Ok((output.await?.code(), stdout))
}
```