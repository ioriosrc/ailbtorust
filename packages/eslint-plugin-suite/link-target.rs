```rust
use rustc_ast::ast::{ItemFn, ItemMod, ModKind};
use rustc_hir::{Expr, Node};
use rustc_lint::{builtin, LintContext, Severity};
use rustc_middle::ty::Ty;

pub fn check_links(cx: &LintContext<'_>) {
    for node in cx.tcx.hir().items() {
        if let ItemFn { id, attrs } = node {
            if attrs.iter().any(|attr| attr.check(builtin::LINK)) {
                for param in id.decl.inputs() {
                    if let Expr::Lit(lit) = param.value {
                        match lit.node {
                            LitKind::Str(s, _) => check_link(cx, s),
                            _ => (),
                        }
                    }
                }
            }
        } else if let ItemMod { name: _, body } = node {
            if body.is_mod() && !body.has_inner_items(ModKind::Empty) {
                for item in &body.body {
                    check_links(cx);
                }
            }
        }
    }
}

fn check_link(cx: &LintContext<'_>, link_str: &str) {
    let mut has_target = false;
    for c in link_str.chars() {
        if c == '#' || c == '?' || c == '&' {
            has_target = true;
            break;
        }
    }
    if !has_target {
        cx.struct_span_err(link_str.span, "Links must specify a target", Severity::Error);
        let suggestions = vec![
            ("Add target=\"_blank\"", "https://www.example.com"),
            ("Add target=\"_self\" (the default)", ""),
        ];
        for suggestion in suggestions {
            cx.struct_span_suggestion(link_str.span, suggestion.0, suggestion.1);
        }
    }
}
```

### Explicação:

1. **Module `check_links`:**
   - Esta função percorre todos os itens do código para encontrar funções e módulos que possam contiver links.
   - Se um item for uma função, ela verifica se há parâmetros que possam ser usados como links.

2. **Function `check_link`:**
   - Esta função é chamada quando um link é encontrado.
   - Ela verifica se o link possui caracteres adicionais (como `#`, `?`, ou `&`), que indicam que ele não especificou um target.
   - Se não houver um target, ela cria uma mensagem de erro e sugestões.

3. **Suggestions:**
   - Sugestões incluem adicionar `target="_blank"` para abrir o link em uma nova aba do navegador ou `target="_self"` (o que é o padrão).

4. **Linter Function:**
   - A função `check_links` é registrada como um linte no linting context (`cx`) para ser chamado durante a análise do código.

### Notas:
- Esta implementação assume que os links são representados por strings literais em Rust.
- As sugestões podem ser ajustadas conforme necessário.