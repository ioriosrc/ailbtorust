```rust
use cytoscape::{Cytoscape, Element};
use cytoscape_dagre::Dagre;
use web_sys::HtmlElement;

struct Graph {
    elements: Vec<Element>,
    style: HashMap<String, String>,
}

impl Graph {
    fn new(elements: Vec<Element>, style: HashMap<String, String>) -> Self {
        Self { elements, style }
    }

    fn render(&self) {
        let mut cy = Cytoscape::new();
        for element in &self.elements {
            cy.add(element.clone());
        }
        for (key, value) in &self.style {
            cy.style().set(key, value);
        }
        cy.layout(Dagre {});
    }
}
```