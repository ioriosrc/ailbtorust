```rust
use crate::components::{PanelSetup, GaugePanel};
use crate::utils::make_fixture;
use storybook::prelude::*;

const PANEL_SETUP_FIXTURE: &str = r#"
{
  "topics": [
    {
      "name": "/data",
      "datatype": "foo_msgs/Bar"
    }
  ],
  "datatypes": new Map([
    ["foo_msgs/Bar", { name: "Bar", definitions: [{ name: "value", type: "float32" }] }]
  ]),
  "frame": {
    "/data": [
      {
        "topic": "/data",
        "receiveTime": { sec: 123, nsec: 456 },
        "message": { value }
      }
    ]
  }
}
"#;

pub fn EmptyState() -> GaugePanel {
  GaugePanel::new()
}

pub fn InvalidValue() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}

pub fn Rainbow() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1,
      colorMode: "colormap",
      colorMap: "rainbow"
    }}
  )
}

pub fn Turbo() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1,
      colorMode: "colormap",
      colorMap: "turbo"
    }}
  )
}

pub fn TurboReverse() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1,
      colorMode: "colormap",
      colorMap: "turbo",
      reverse: true
    }}
  )
}

pub fn CustomGradient() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1,
      colorMode: "gradient",
      gradient: ["#ec9a57", "#65c6ff"]
    }}
  )
}

pub fn CustomGradientReverse() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1,
      colorMode: "gradient",
      gradient: ["#ec9a57", "#65c6ff"],
      reverse: true
    }}
  )
}

pub fn MinValue() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}

pub fn MaxValue() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}

pub fn TooLow() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}

pub fn TooHigh() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}

pub fn CustomRange() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 5,
      maxValue: 7
    }}
  )
}

pub fn MessagePathWithFilter() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: `/data{id=="b"}.value`,
      minValue: 0,
      maxValue: 4
    }}
  )
}

pub fn StringValue() -> GaugePanel {
  GaugePanel::new(
    override_config! {{
      path: "/data.value",
      minValue: 0,
      maxValue: 1
    }}
  )
}
```