```rust
fn main() {
    let github = Github {
        silent: true,
        headers: vec![
            Header {
                source: "/(.*)".to_string(),
                headers: vec![
                    Header {
                        key: "X-Frame-Options".into(),
                        value: "DENY".into(),
                    },
                    Header {
                        key: "Referrer-Policy".into(),
                        value: "origin".into(),
                    },
                    Header {
                        key: "Content-Security-Policy".into(),
                        value: "base-uri 'self';".into(),
                    },
                    Header {
                        key: "X-Content-Type-Options".into(),
                        value: "nosniff".into(),
                    },
                    Header {
                        key: "Cross-Origin-Opener-Policy".into(),
                        value: "same-origin".into(),
                    },
                    Header {
                        key: "Cross-Origin-Embedder-Policy".into(),
                        value: "credentialless".into(),
                    },
                ],
            },
        ],
    };

    // Handle the github settings as needed
}
```

```rust
struct Github {
    silent: bool,
    headers: Vec<Header>,
}

struct Header {
    source: String,
    headers: Vec<(String, String)>,
}
```
Neste código Rust, não há necessidade de conversão explícita de TypeScript para Rust. As estruturas e os métodos são semelhantes.