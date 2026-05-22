```rust
use leaflet::{Map, LatLngBounds};
use mapbox_geocoder::GeocodeOptions;

use foxglove_msgs::{
    FoxgloveMessages,
    types::{NavSatFix, NavSatFixPositionCovarianceType},
};

pub type Point = (f64, f64);

pub type Matrix3x3 = [f64; 9];

// https://docs.ros.org/en/api/sensor_msgs/html/msg/NavSatFix.html

#[derive(Debug)]
pub enum NavSatFixStatus {
    NoFix,
    Fix,
    SbasFix,
    GbasFix,
}

#[derive(Debug)]
pub enum NavSatFixService {
    Gps,
    Glonass,
    Compass,
    Galileo,
}

pub type NavSatFixMsg = FoxgloveMessages::NavSatFix;

pub type MapPanelMessage =
    | foxglove_msgs::MessageEvent<FoxgloveMessages::GeoJSON>
    | foxglove_msgs::MessageEvent<NavSatFixMsg>;

pub type FilteredPointLayerArgs = {
    map: Map;
    bounds: LatLngBounds;
    color: String;
    hover_color: String;
    show_accuracy: bool;
    nav_sat_message_events: Vec<foxglove_msgs::MessageEvent<NavSatFixMsg>>;
    on_hover: Option<Box<dyn FnMut(Option<&foxglove_msgs::MessageEvent<NavSatFixMsg>>) -> ()>>,
    on_click: Option<Box<dyn FnMut(&foxglove_msgs::MessageEvent<NavSatFixMsg>) -> ()>>,
};
```