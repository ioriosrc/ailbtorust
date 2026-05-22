```rust
use std::collections::HashMap;

struct CustomTypography {
    font_monospace: String,
    font_sans_serif: String,
    font_feature_settings: String,
}

// We explicitly avoid fallback fonts (such as 'monospace') here to work around a bug in
// Chrome/Chromium on Windows that causes crashes when multiple Workers try to access fonts that
// have not yet been loaded. There is a race against the internal DirectWrite font cache which
// ends up crashing in DWriteFontFamily::GetFirstMatchingFont() or DWriteFont::Create().
//
// https://bugs.chromium.org/p/chromium/issues/detail?id=1261577
const font_sans_serif: &str = "'Inter'";
const font_monospace: &str = "'IBM Plex Mono'";

const font_feature_settings: &str = [
    "'tnum'", // enable tabular-numerals
    "'calt' 0", // disable contextual-alternates",
].join(",");

struct HeadingFontStyles {
    font_feature_settings: String,
    letter_spacing: f64,
    font_weight: u8,
}

const heading_font_styles: HeadingFontStyles = HeadingFontStyles {
    font_feature_settings: font_feature_settings.to_string(),
    letter_spacing: -0.025,
    font_weight: 800,
};

struct SubtitleFontStyles {
    font_feature_settings: String,
    font_weight: u8,
}

const subtitle_font_styles: SubtitleFontStyles = SubtitleFontStyles {
    font_feature_settings: font_feature_settings.to_string(),
    font_weight: 500,
};

impl CustomTypography {
    pub fn new() -> Self {
        Self {
            font_monospace,
            font_sans_serif,
            font_feature_settings,
        }
    }
}

fn main() {
    let mut typography = CustomTypography::new();
    typography.font_sans_serif = font_sans_serif.to_string();
    typography.font_monospace = font_monospace.to_string();
    typography.font_feature_settings = font_feature_settings.to_string();

    let typography_variants: HashMap<String, HashMap<&str, String>> = HashMap::from([
        (
            "h1".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "h2".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "h3".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "h4".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "h5".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "h6".to_string(),
            [("font-feature-settings".to_string(), heading_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "subtitle1".to_string(),
            [("font-feature-settings".to_string(), subtitle_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "subtitle2".to_string(),
            [("font-feature-settings".to_string(), subtitle_font_styles.font_feature_settings)].iter().cloned().collect()
        ),
        (
            "body1".to_string(),
            [("font-feature-settings".to_string(), typography_variants["h1"].get("font-feature-settings").unwrap()).to_string()].iter().cloned().collect()
        ),
        (
            "body2".to_string(),
            [("font-feature-settings".to_string(), typography_variants["h1"].get("font-feature-settings").unwrap()).to_string()].iter().cloned().collect()
        ),
        (
            "button".to_string(),
            [
                ("font-feature-settings".to_string(), typography_variants["h1"].get("font-feature-settings").unwrap()).to_string()],
                ("letter-spacing".to_string(), &heading_font_styles.letter_spacing.to_string()),
                ("fontWeight".to_string(), &heading_font_styles.font_weight.to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        ),
        (
            "overline".to_string(),
            [
                ("font-feature-settings".to_string(), typography_variants["h1"].get("font-feature-settings").unwrap()).to_string()],
                ("letter-spacing".to_string(), &heading_font_styles.letter_spacing.to_string()),
                ("lineHeight".to_string(), "1.5"),
            ]
            .iter()
            .cloned()
            .collect()
        ),
    ]);

    println!("{:?}", typography_variants);
}
```