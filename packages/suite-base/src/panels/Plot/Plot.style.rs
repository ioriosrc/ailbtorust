```rust
use styled_components::{css, styled};

styled! {
  .tooltip {
    max-width: none;
  }

  .resetZoomButton {
    pointer-events: none;
    position: absolute;
    display: flex;
    justify-content: flex-end;
    padding-inline: ${p => p.theme.spacing(1)};
    right: 0;
    left: 0;
    bottom: 0;
    width: "100%";
    paddingBottom: ${p => p.theme.spacing(4)};

    & .${styled_components::buttonClasses.root)} {
      pointer-events: auto;
    }
  }

  .canvasDiv {
    width: "100%";
    height: "100%";
    overflow: hidden;
    cursor: crosshair;
  }

  .verticalBarWrapper {
    width: "100%";
    height: "100%";
    overflow: hidden;
    position: relative;
  }
}
```