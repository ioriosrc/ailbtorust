```rust
use styled_components::{css, styled};
use styled_system::Breakpoint;
use styled_system::Theme;

pub fn create_styles(theme: &Theme) -> impl Fn(&Theme) -> styled_components::StyledComponentsBuilder<Theme> {
    styled! {
        .logo {
            width: 212px;
            height: "auto";
            margin-left: theme.spacing(-1);
        }

        .grid {
            @media (min-width: ${theme.breakpoints.up("md")}) {
                display: grid;
                grid-template-areas: "header spacer" "content sidebar";
                grid-template-rows: content auto;
                grid-template-columns: 1fr 375px;
            }
        }

        .header {
            padding: theme.spacing(6);
            grid-area: header;

            @media (max-width: ${theme.breakpoints.values.sm}) {
                padding: theme.spacing(4);
            }

            &::before {
                content: "";
                display: none;
            }
        }

        .content {
            padding: theme.spacing(0, 6, 6);
            overflow: hidden;
            grid-area: content;

            @media (max-width: ${theme.breakpoints.values.sm}) {
                padding: theme.spacing(0, 4, 4);
            }

            &::before {
                content: "";
                display: none;
            }
        }

        .spacer {
            grid-area: spacer;
            background-color: tinycolor(theme.palette.text.primary).setAlpha(0.04).toRgbString();

            @media (max-width: ${theme.breakpoints.values.sm}) {
                display: none;
            }
        }

        .sidebar {
            grid-area: sidebar;
            background-color: tinycolor(theme.palette.text.primary).setAlpha(0.04).toRgbString();
            padding: theme.spacing(0, 5, 5);

            @media (max-width: ${theme.breakpoints.values.sm}) {
                padding: theme.spacing(4);
            }
        }

        .button {
            white-space: nowrap;
            text-overflow: ellipsis;
            overflow: hidden;
        }

        .connection-button {
            text-align: left;
            justify-content: flex-start;
            padding: theme.spacing(2, 3);
            gap: theme.spacing(1.5);
            border-color: theme.palette.divider;

            &::before {
                content: "";
                display: none;
            }
        }

        .recent-list-item-button {
            overflow: hidden;
            color: theme.palette.primary.main;

            &:hover {
                background-color: "transparent";
                color: theme.palette.primary[theme.palette.mode === "dark" ? "light" : "dark"];
            }
        }

        .recent-source-secondary {
            color: "inherit";
        }
    }
}
```

Este código é um exemplo de como converter o código TypeScript/React para Rust funcional usando styled-components. É importante notar que styled-components não tem uma representação exata em Rust, mas você pode usar os recursos disponíveis para criar interfaces estilizadas no Rust.