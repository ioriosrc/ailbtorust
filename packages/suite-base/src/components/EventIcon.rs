```rust
use svg::node::document;
use svg::node::element::{circle, path};
use svg::Document;

fn main() {
    let mut doc = Document::new();
    let circle_node = circle::builder()
        .cx(18.03)
        .cy(3.03)
        .r(1.0)
        .fill("black")
        .build();

    let path_node = path::builder()
        .d("M18.03,3.03H6.03c-.55,0-1,.45-1,1v3h0v9.42c0,.36,.18,.69,.5,.87l6.5,3.71,6.5-3.71c.31-.18,.5-.51,.5-.87V4.03c0-.55-.45-1-1-1Z")
        .fill("black")
        .build();

    doc.append(circle_node);
    doc.append(path_node);

    println!("{}", doc.to_string());
}
```