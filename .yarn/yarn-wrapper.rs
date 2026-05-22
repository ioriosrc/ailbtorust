```rust
fn main() {
    use std::env;

    if env::var_os("COREPACK_ROOT").is_none() {
        eprintln!("This repository uses corepack. Enable corepack by running `corepack enable`");
        eprintln!("Learn more at: https://nodejs.org/api/corepack.html");
        eprintln("");
        eprintln(
            "If you have run `corepack enable` and still see this error, you have likely installed yarn globally or using your system package manager. Your installed version of yarn is superseding corepack's version. Delete or uninstall your version of yarn to use the corepack version.",
        );
        std::process::exit(1);
    }

    // Increases the v8 old memory space (effectively increasing v8 heap space beyond the 2GB default)
    env::set_var("NODE_OPTIONS", "--max-old-space-size=6144");

    let yarncmd = path::join(env::var_os("COREPACK_ROOT").unwrap(), "dist", "yarn.js");
    let _ = std::process::Command::new(&yarncmd).status();
}
```