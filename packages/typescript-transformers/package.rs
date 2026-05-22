```rust
use std::fs;
use std::path::Path;

fn main() {
    if let Err(e) = compile_swc("src/index.ts") {
        println!("Error: {}", e);
    }
}

fn compile_swc(src_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("tswc");
    cmd.arg("--target").arg("esnext");
    cmd.arg("--out-dir").arg("dist");
    cmd.args(&["src", src_path]);
    let output = cmd.output()?;

    if !output.status.success() {
        Err(format!("Failed to compile code: {:?}", output))
    } else {
        Ok(())
    }
}
```