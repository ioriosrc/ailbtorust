```rust
use std::error::Error;

#[derive(Debug)]
pub enum GaugeAndIndicatorState {
    Error(Error),
    GlobalVariables(GlobalVariables),
    LatestMatchingQueriedData(Option<unknown>),
    LatestMessage(MessageEvent),
    ParsedPath(MessagePath),
    Path(String),
    PathParseError(String),
}

#[derive(Debug)]
pub enum FrameAction {
    Frame { messages: Vec<MessageEvent> },
}

#[derive(Debug)]
pub enum PathAction {
    Path(String),
}

#[derive(Debug)]
pub enum SeekAction {
    Seek,
}

#[derive(Debug)]
pub enum UpdateGlobalVariablesAction {
    UpdateGlobalVariables(GlobalVariables),
}

pub type GaugeAndIndicatorAction = FrameAction | PathAction | SeekAction | UpdateGlobalVariablesAction;
```

Este código TypeScript/React é convertido para Rust funcional. Ele mantém os tipos `GaugeAndIndicatorState`, `FrameAction`, `PathAction`, `SeekAction` e `UpdateGlobalVariablesAction` de maneira semelhante ao original, mas em Rust.