```rust
use rustc::hir::{ExprKind, FnSig};
use rustc::lint::builtin::RestrictedCall;
use rustc::lint::LintStore;
use rustc::lint::LateContext;
use rustc::lint::{EarlyContext, LateLintPass};
use rustc::mir::visit::*;
use rustc::ty::Ty;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub struct TssReactNameTransformer;

impl LateLintPass for TssReactNameTransformer {
    fn check_late(
        &mut self,
        ctx: &LateContext<'_>,
        _: &EarlyContext<'_>,
        tcx: TyCtxt<'_>,
    ) -> Result<(), &'static str> {
        let mut visitor = TssReactNameVisitor { ctx, tcx };
        visit::walk_crate(&mut visitor);
        Ok(())
    }
}

struct TssReactNameVisitor<'tcx> {
    ctx: LateContext<'tcx>,
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> LintPass for TssReactNameVisitor<'tcx> {
    fn check_late(
        &mut self,
        ctx: &LateContext<'_>,
        _: &EarlyContext<'_>,
        tcx: TyCtxt<'_>,
    ) -> Result<(), &'static str> {
        let mut visitor = TssReactNameVisitor { ctx, tcx };
        visit::walk_crate(&mut visitor);
        Ok(())
    }
}

impl<'tcx> Visit for TssReactNameVisitor<'tcx> {
    fn visit_call_expr(&mut self, e: &CallExpr<'_>) {
        if let CallExprKind::StaticFnCall { func, .. } = &e.kind {
            if let Some(StaticFnApp { ident }) = func.body.as_ref().map(|body| body.expr) {
                if let Ident { name } = ident {
                    if name == "makeStyles" && is_imported_from(tcx, name.to_string(), e.ty) {
                        self.replace_with_tss_react_name(e);
                    }
                }
            }
        }
        visit::visit_call_expr(self, e);
    }

    fn replace_with_tss_react_name(&mut self, call: &CallExpr<'_>) {
        let module = call.module();
        if let Some(module) = module {
            let sanitized_filename = format!("{}{}", module.to_string().replace('/', '_'), call.ident.to_string());
            let name_arg = self.ctx.resolve_ident(&sanitized_filename);
            if let Ok(name_arg) = name_arg {
                self.replace_with_object_literal(call, name_arg);
            }
        }
    }

    fn replace_with_object_literal(&mut self, call: &CallExpr<'_>, name_arg: NameArg<'tcx>) {
        let mut object_literal = ObjectLit::new();
        object_literal.push_property(Literal::String(sanitized_filename));
        let new_call = CallExpr {
            ident: Ident::from_str("makeStyles"),
            span: call.span,
            kind: CallExprKind::StaticFnCall {
                func: StaticFnApp {
                    ident: name_arg.ident(),
                    args: vec![ObjectLit::from(object_literal)],
                },
                ty: call.ty,
            },
        };
        self.ctx.replace_node_with(&call, &new_call);
    }
}
```