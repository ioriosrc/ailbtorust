```rust
use web_sys::FontFace;

async fn load_default_font() -> Result<FontFace, Box<dyn std::error::Error>> {
    let response = fetch(PlexMono);
    let array_buffer = await response.arrayBuffer();
    let font_face = FontFace::new("IBM Plex Mono", array_buffer)?;

    if web_sys::window().is_worker() {
        web_sys::globalScope()
            .fonts
            .add(&font_face)
            .map_err(|e| Box<dyn std::error::Error>::from(e))?;
    } else {
        document::fonts.add(&font_face)?;
    }

    Ok(font_face)
}
```