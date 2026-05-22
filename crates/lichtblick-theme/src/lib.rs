// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Theme system for Lichtblick, providing colors, typography, and spacing.

use serde::{Deserialize, Serialize};

/// Complete theme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub palette: Palette,
    pub typography: Typography,
    pub spacing: Spacing,
}

/// Color palette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub primary: ColorScale,
    pub secondary: ColorScale,
    pub success: ColorScale,
    pub warning: ColorScale,
    pub error: ColorScale,
    pub info: ColorScale,
    pub background: BackgroundColors,
    pub text: TextColors,
    pub divider: String,
}

/// Color scale with main, light, dark, and contrast text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScale {
    pub main: String,
    pub light: String,
    pub dark: String,
    pub contrast_text: String,
}

/// Background colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    pub default: String,
    pub paper: String,
}

/// Text colors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
}

/// Typography settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_size_base: f32,
    pub font_weight_regular: u16,
    pub font_weight_medium: u16,
    pub font_weight_bold: u16,
}

/// Spacing system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub unit: f32,
}

impl Spacing {
    /// Get spacing value for the given multiplier.
    pub fn get(&self, multiplier: f32) -> f32 {
        self.unit * multiplier
    }
}

/// Create the dark theme.
pub fn dark_theme() -> Theme {
    Theme {
        palette: Palette {
            primary: ColorScale {
                main: "#4dabf5".into(),
                light: "#80c4f8".into(),
                dark: "#2196f3".into(),
                contrast_text: "#000000".into(),
            },
            secondary: ColorScale {
                main: "#ce93d8".into(),
                light: "#f3e5f5".into(),
                dark: "#ab47bc".into(),
                contrast_text: "#000000".into(),
            },
            success: ColorScale {
                main: "#66bb6a".into(),
                light: "#81c784".into(),
                dark: "#388e3c".into(),
                contrast_text: "#000000".into(),
            },
            warning: ColorScale {
                main: "#ffa726".into(),
                light: "#ffb74d".into(),
                dark: "#f57c00".into(),
                contrast_text: "#000000".into(),
            },
            error: ColorScale {
                main: "#f44336".into(),
                light: "#e57373".into(),
                dark: "#d32f2f".into(),
                contrast_text: "#ffffff".into(),
            },
            info: ColorScale {
                main: "#29b6f6".into(),
                light: "#4fc3f7".into(),
                dark: "#0288d1".into(),
                contrast_text: "#000000".into(),
            },
            background: BackgroundColors {
                default: "#121212".into(),
                paper: "#1e1e1e".into(),
            },
            text: TextColors {
                primary: "#ffffff".into(),
                secondary: "rgba(255,255,255,0.7)".into(),
                disabled: "rgba(255,255,255,0.5)".into(),
            },
            divider: "rgba(255,255,255,0.12)".into(),
        },
        typography: Typography {
            font_family: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".into(),
            font_size_base: 14.0,
            font_weight_regular: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
        },
        spacing: Spacing { unit: 8.0 },
    }
}

/// Create the light theme.
pub fn light_theme() -> Theme {
    Theme {
        palette: Palette {
            primary: ColorScale {
                main: "#1976d2".into(),
                light: "#42a5f5".into(),
                dark: "#1565c0".into(),
                contrast_text: "#ffffff".into(),
            },
            secondary: ColorScale {
                main: "#9c27b0".into(),
                light: "#ba68c8".into(),
                dark: "#7b1fa2".into(),
                contrast_text: "#ffffff".into(),
            },
            success: ColorScale {
                main: "#2e7d32".into(),
                light: "#4caf50".into(),
                dark: "#1b5e20".into(),
                contrast_text: "#ffffff".into(),
            },
            warning: ColorScale {
                main: "#ed6c02".into(),
                light: "#ff9800".into(),
                dark: "#e65100".into(),
                contrast_text: "#ffffff".into(),
            },
            error: ColorScale {
                main: "#d32f2f".into(),
                light: "#ef5350".into(),
                dark: "#c62828".into(),
                contrast_text: "#ffffff".into(),
            },
            info: ColorScale {
                main: "#0288d1".into(),
                light: "#03a9f4".into(),
                dark: "#01579b".into(),
                contrast_text: "#ffffff".into(),
            },
            background: BackgroundColors {
                default: "#fafafa".into(),
                paper: "#ffffff".into(),
            },
            text: TextColors {
                primary: "rgba(0,0,0,0.87)".into(),
                secondary: "rgba(0,0,0,0.6)".into(),
                disabled: "rgba(0,0,0,0.38)".into(),
            },
            divider: "rgba(0,0,0,0.12)".into(),
        },
        typography: Typography {
            font_family: "Inter, -apple-system, BlinkMacSystemFont, sans-serif".into(),
            font_size_base: 14.0,
            font_weight_regular: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
        },
        spacing: Spacing { unit: 8.0 },
    }
}
