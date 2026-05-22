```rust
use cdk::{
    layout::Flex,
    node::{Node},
    style::Style,
};

struct GraphMutation {
    fit: () -> (),
    reset_user_pan_zoom: () -> (),
}

type Props = {
    elements: Vec<cdk::node::Element>,
    rank_dir: String,
    graph_ref: cdk::react::Ref<Node>,
};

fn Graph(props: Props) -> Node {
    let cy = create_cytoscape();
    let graph_ref = props.graph_ref;

    // indicates that a user has manually panned/zoomed the viewport
    // we avoid performing actions like automatic fit when this happens.
    let mut user_pan_zoom = false;

    useEffect(() => {
        if !graph_ref.is_mounted() {
            panic!("Graph ref must be available on first render");
        }

        let cy = &cy;
        let graph_ref = graph_ref.clone();

        cy.on("viewport", |_| user_pan_zoom.set(true));

        props.graph_ref.current = Some(GraphMutation {
            fit: || {
                user_pan_zoom.set(false);
                cy.fit();
            },
            reset_user_pan_zoom: || {
                user_pan_zoom.set(false);
            },
        });

        return || {
            let graph = &cy;
            graph.destroy();
        };
    }, [props.graph_ref]);

    let { elements, rank_dir } = props;

    useEffect(() => {
        if (!cy.is_mounted()) {
            return;
        }

        cy.batch(|| {
            cy.elements().remove();
            cy.add(elements);
            cy
                .elements()
                .make_layout(DAG_LAYOUT)
                .run();
        });

        if (!user_pan_zoom) {
            cy.fit();
        }
    }, [elements, rank_dir]);

    useEffect(() => {
        cy.style(props.style.clone());
    }, [props.style]);

    Node::new()
        .with_id("graph")
        .with_content(create_div())
        .with_props({
            flex: Flex {
                direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: "100%",
                height: "100%",
            },
            style: Style {
                background_color: "#fff",
                border_width: 2.0,
                border_style: BorderStyle::Solid,
                border_color: "#ccc",
            },
        })
}
```