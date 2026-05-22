```rust
use styled_components::css;

fn GlobalStyle() -> css! {
  "*": {
    box_sizing: "border-box";
  },

  body, html: {
    width: "100%",
    height: "100%",
    padding: "0",
    margin: "0",

    @media (prefers-color-scheme: dark) {
      background: "#333";
    },
  },

  body: {
    padding: "{body_padding}px !important", // important for Storybook
    minWidth: 150,
    fontFamily: "'ui-sans-serif', -apple-system, BlinkMacSystemFont, sans-serif",

    @media (prefers-color-scheme: dark) {
      color: "#fff";
    },
  },

  "pre, code, tt": {
    fontFamily: "'ui-monospace', Menlo, Monaco, monospace",
  },

  a: {
    color: "#476ebd",

    @media (prefers-color-scheme: dark) {
      color: "#99b5ed";
    },
  }
}
```