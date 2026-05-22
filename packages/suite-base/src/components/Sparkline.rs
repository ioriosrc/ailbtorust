```rust
use std::f64;

use react_native::{react_native as rn, Component, View, ViewProps};
use tss_react::{make_styles, use_theme};

type SparklinePoint = (f64, f64);

type SparklineProps = ViewProps + 'static;

const SPARKLINE_COLOR = "black";

fn draw(
    points: &[SparklinePoint],
    maximum: f64,
    time_range: f64,
    now_stamp: f64,
    context: &mut rn::CanvasContext,
    width: u32,
    height: u32,
) {
    let mut maxValue = 0.0;
    for &(_, y) in points {
        maxValue = std::cmp::max(maxValue, y);
    }

    context.clear(Color::from_rgb(0x69, 0x84, 0x93));
    context.set_stroke_color(Color::from_rgb(0, 0, 0));

    let mut first = true;
    for &(x, y) in points {
        let x = ((time_range + x - now_stamp) / time_range) * width as f64;
        let y = (1.0 - y / maxValue) * height as f64;

        if first {
            context.move_to(x as i32, y as i32);
            first = false;
        } else {
            context.line_to(x as i32, y as i32);
        }
    }

    context.stroke();
}

fn Sparkline(props: SparklineProps) -> Component {
    let { width, height } = props.style;

    let theme = use_theme();

    let draw_callback = useCallback(
        move |context: &mut rn::CanvasContext| {
            draw(
                &props.children,
                props.maximum.unwrap_or(0.0),
                props.time_range.unwrap_or(1.0),
                props.now_stamp.unwrap_or_now(),
                context,
                width as u32,
                height as u32,
            );
        },
        [&props.children, &props.maximum, &props.now_stamp, &props.time_range],
    );

    View::new()
        .with_width(width)
        .with_height(height)
        .with_children(move || {
            view! {
                <rn::AutoSizingCanvas draw={draw_callback} />
            }
        })
}

fn main() -> rn::Result<(), Box<dyn std::error::Error>> {
    rn::Application::new(Sparkline {})
}
```