```rust
use monaco::theme::{StandaloneThemeData, Theme};

const theme: Theme = StandaloneThemeData {
    base: String::from("vs-dark"),
    inherit: true,
    rules: vec![
        Rule {
            foreground: String::from("#6c6783"),
            token: String::from("comment"),
        },
        Rule {
            foreground: String::from("#eba800"),
            token: String::from("string"),
        },
        Rule {
            foreground: String::from("#9987ff"),
            token: String::from("constant.numeric"),
        },
        Rule {
            foreground: String::from("#9987ff"),
            token: String::from("constant.language"),
        },
        Rule {
            foreground: String::from("#9987ff"),
            token: String::from("constant.character"),
        },
        Rule {
            foreground: String::from("#9987ff"),
            token: String::from("constant.other"),
        },
        Rule {
            foreground: String::from("#e05ffa"),
            token: String::from("keyword"),
        },
        Rule {
            foreground: String::from("#e05ffa"),
            token: String::from("storage"),
        },
        Rule {
            foreground: String::from("#45a5ff"),
            fontStyle: FontStyle::Italic,
            token: String::from("storage.type"),
        },
        Rule {
            foreground: String::from("#6bd66f"),
            fontStyle: FontStyle::Underline,
            token: String::from("entity.name.class"),
        },
        Rule {
            foreground: String::from("#6bd66f"),
            fontStyle: FontStyle::Italic Underline,
            token: String::from("entity.other.inherited-class"),
        },
        Rule {
            foreground: String::from("#6bd66f"),
            token: String::from("entity.name.function"),
        },
        Rule {
            foreground: String::from("#fc8942"),
            fontStyle: FontStyle::Italic,
            token: String::from("variable.parameter"),
        },
        Rule {
            foreground: String::from("#db3553"),
            token: String::from("entity.name.tag"),
        },
        Rule {
            foreground: String::from("#6bd66f"),
            token: String::from("entity.other.attribute-name"),
        },
        Rule {
            foreground: String::from("#45a5ff"),
            token: String::from("support.function"),
        },
        Rule {
            foreground: String::from("#45a5ff"),
            token: String::from("support.constant"),
        },
        Rule {
            foreground: String::from("#45a5ff"),
            fontStyle: FontStyle::Italic,
            token: String::from("support.type"),
        },
        Rule {
            foreground: String::from("#45a5ff"),
            fontStyle: FontStyle::Italic,
            token: String::from("support.class"),
        },
        Rule {
            foreground: String::from("#f0f0f0"),
            background: String::from("#ff6b82"),
            token: String::from("invalid"),
        },
        Rule {
            foreground: String::from("#f0f0f0"),
            background: String::from("#6858f5"),
            token: String::from("invalid.deprecated"),
        },
    ],
    colors: vec![
        ("editor.foreground".to_string(), "#F7F7F3C4"),
        ("editor.background".to_string(), "#08080A"),
        ("editor.selectionBackground".to_string(), "#F7F7F326"),
        ("editor.lineHighlightBackground".to_string(), "#F7F7F31A"),
        ("editorCursor.foreground".to_string(), "#F7F7F3C4"),
        ("editorWhitespace.foreground".to_string(), "#3B3A32"),
    ],
};

fn main() {
    // Usage of the theme
}
```