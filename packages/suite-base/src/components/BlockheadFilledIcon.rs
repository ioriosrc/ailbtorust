```rust
use svg::node::{element, Node};
use svg::text::Text;
use svg::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::new();

    let rect = element::rect()
        .set("x", "8.07")
        .set("y", "5.38")
        .set("width", "7.87")
        .set("height", "7.87")
        .set("rx", "2");

    let path = element::path()
        .set("d", "M12,1.53A10.5,10.5,0,1,0,22.5,12,10.51,10.51,0,0,0,12,1.53ZM18.41,19l-1.67-2.61a2.47,2.47,0,0,0-2-1H9.26a2.48,2.48,0,0,0-2,1L5.59,19a9.5,9.5,0,1,1,12.82,0Z")
        .set("fill", "currentColor");

    let group = element::g().append(&rect).append(&path);

    doc.append(&group);

    // Add text if needed
    let text = Text::new("Filled Icon")
        .set_x(40)
        .set_y(50);

    doc.append(&text);

    doc.save("blockhead_filled_icon.svg")?;

    Ok(())
}
```