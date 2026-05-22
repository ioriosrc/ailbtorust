```rust
use react::prelude::*;

#[function_component(Sparkline)]
fn Sparkline(props: &Props) -> Html {
    let points = props.points;
    let width = props.width;
    let height = props.height;
    let time_range = props.time_range;
    let now_stamp = props.now_stamp;

    html! {
        <div style={{ padding: "8px" }}>
            <SparklineComponent points=points width=width height=height time_range=time_range now_stamp=now_stamp />
        </div>
    }
}

#[function_component(WithExplicitMaximumOf200)]
fn WithExplicitMaximumOf200(props: &Props) -> Html {
    let points = props.points;
    let width = props.width;
    let height = props.height;
    let time_range = props.time_range;
    let now_stamp = props.now_stamp;

    html! {
        <div style={{ padding: "8px" }}>
            <SparklineComponent points=points width=width height=height time_range=time_range now_stamp=now_stamp maximum=200 />
        </div>
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Props {
    points: Vec<SparklinePoint>,
    width: usize,
    height: usize,
    time_range: usize,
    now_stamp: usize,
}

type SparklineComponent = fn(&Props) -> Html;

fn main() {
    // Example usage
    let props_standard = Props {
        points: vec![
            SparklinePoint { value: 5, timestamp: 10 },
            SparklinePoint { value: 50, timestamp: 30 },
            SparklinePoint { value: 30, timestamp: 60 },
            SparklinePoint { value: 100, timestamp: 100 },
        ],
        width: 300,
        height: 100,
        time_range: 100,
        now_stamp: 100,
    };

    let props_with_maximum = Props {
        points: vec![
            SparklinePoint { value: 5, timestamp: 10 },
            SparklinePoint { value: 50, timestamp: 30 },
            SparklinePoint { value: 30, timestamp: 60 },
            SparklinePoint { value: 100, timestamp: 100 },
        ],
        width: 300,
        height: 100,
        time_range: 100,
        now_stamp: 100,
    };

    let standard = SparklineComponent::new(Standard);
    let with_maximum = SparklineComponent::new(WithExplicitMaximumOf200);

    // Render the components
    // Assuming you have a way to render these components in your Rust application
}
```