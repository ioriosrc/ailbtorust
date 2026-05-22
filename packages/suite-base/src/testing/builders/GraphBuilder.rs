```rust
use std::rc::{Rc, Weak};
use cytoscape_rs as cy;

pub type GraphMutation = Rc<dyn dyn Fn(&cy::CyData)>>;

pub struct GraphBuilder {
    style: Vec<(String, HashMap<String, String>)>,
    elements: Vec<cy::ElementDefinition>,
    rank_dir: String,
    graph_ref: Weak<dyn Fn(&cy::CyData)>>,
}

impl GraphBuilder {
    pub fn element(props: Option<HashMap<String, String>>) -> cy::ElementDefinition {
        let props = props.unwrap_or_else(|| Default::default());
        cy::ElementDefinition::new()
            .data(cy::NodeData::with_id_and_label(props["id"].as_str(), props["label"].as_str()))
    }

    pub fn elements(count: usize) -> Vec<cy::ElementDefinition> {
        (0..count).map(|_| Self::element(None)).collect()
    }

    pub fn stylesheet_style(props: Option<HashMap<String, String>>) -> cy::StylesheetStyle {
        let props = props.unwrap_or_else(|| Default::default());
        cy::StylesheetStyle::new()
            .selector(&props["selector"])
            .style(cy::CssNode { ..Default::default() })
    }

    pub fn stylesheet_styles(count: usize) -> Vec<cy::StylesheetStyle> {
        (0..count).map(|_| Self::stylesheet_style(None)).collect()
    }

    pub fn props(props: Option<HashMap<String, String>>) -> GraphProps {
        let props = props.unwrap_or_else(|| Default::default());
        GraphProps {
            style: Self::stylesheet_styles(props["style"].as_str()),
            elements: Self::elements(props["elements"].as_usize()),
            rank_dir: BasicBuilder.sample(["TB", "LR"]),
            graph_ref: Weak::new(),
        }
    }
}

struct GraphProps {
    style: Vec<(String, HashMap<String, String>)>,
    elements: Vec<cy::ElementDefinition>,
    rank_dir: String,
    graph_ref: Weak<dyn Fn(&cy::CyData)>>,
}
```