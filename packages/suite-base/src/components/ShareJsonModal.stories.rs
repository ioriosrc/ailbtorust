```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use wasm_bindgen::prelude::*;
use yew::{html, prelude::*};

#[function_component(ShareJsonModal)]
fn share_json_modal(props: &ShareJsonModalProps) -> Html {
    html! {
        <div class="modal">
            <header>
                <h2>{ props.title }</h2>
            </header>
            <main>
                <textarea
                    id="share-json-input"
                    placeholder="Paste JSON here"
                    oninput={props.on_change}
                />
            </main>
            <footer>
                <button onclick={props.on_request_close}>Close</button>
            </footer>
        </div>
    }
}

#[function_component(Standard)]
fn standard() -> Html {
    html! {
        <ShareJsonModal title="Foo" onChange={() => {}} onRequestClose={() => {}} initialValue="" />
    }
}

#[function_component(StandardLight)]
fn standard_light() -> Html {
    html! {
        <ShareJsonModal title="Foo" onChange={() => {}} onRequestClose={() => {}} initialValue="" />
    }
}

#[function_component(JSON)]
fn json() -> Html {
    html! {
        <ShareJsonModal title="Foo" onChange={() => {}} onRequestClose={() => {}} initialValue={{ foo: "bar", baz: "qux" }} />
    }
}

#[function_component(SubmittingInvalidLayout)]
fn submitting_invalid_layout() -> Html {
    html! {
        <ShareJsonModal title="Foo" onChange={() => {}} onRequestClose={() => {}} initialValue="" />
    }
}
```

Note that this Rust code uses wasm-bindgen to interact with the DOM and Yew for building the React-like components. The `wasm_bindgen` crate is used to compile Rust functions into JavaScript, while the `yew` crate provides a simple framework for building user interfaces in JavaScript.