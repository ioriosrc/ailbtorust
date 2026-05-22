```rust
use structopt::StructOpt;
use std::fs;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    src: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    fs::copy(&args.src, "dist/index.ts")?;
    Ok(())
}
```