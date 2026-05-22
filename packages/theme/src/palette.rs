```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaletteOptions {
    pub name: String;
    pub mode: &'static str;
    pub tonal_offset: f64;

    #[serde(rename = "appBar")]
    pub appBar: AppBar;

    pub primary: PrimaryColor;
    pub secondary: SecondaryColor;
    pub error: ErrorColor;
    pub warning: WarningColor;
    pub success: SuccessColor;
    pub info: InfoColor;

    pub text: TextColor;
    pub divider: Dividers;

    #[serde(rename = "background")]
    pub background: Background;

    pub grey: GreyColors;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppBar {
    main: String,
    primary: String,
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WarningColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoColor {
    main: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextColor {
    primary: String,
    secondary: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dividers {
    default: String,
    paper: String,
    menu: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Background {
    default: String,
    paper: String,
    menu: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GreyColors {
    50: String,
    100: String,
    200: String,
    300: String,
    400: String,
    500: String,
    600: String,
    700: String,
    800: String,
    900: String,
    A100: String,
    A200: String,
    A400: String,
    A700: String,
}
```