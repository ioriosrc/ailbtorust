```rust
use rustc_lint::{clippy::useless_import, utils};

fn main() {
    let rules = vec![
        rustc_lint::Rule {
            name: "use_different_package",
            message: "Use different package for lodash-es and ramda",
            level: rustc_lint::Level::Warning,
            checker: Box::new(use_different_package),
        },
        rustc_lint::Rule {
            name: "use_namespace_import",
            message: "Use namespace import for lodash-es and ramda",
            level: rustc_lint::Level::Warning,
            checker: Box::new(use_namespace_import),
        },
    ];

    let mut rule_tester = utils::RuleTester::new(rules);

    let valid_code = vec![
        r#"
import * as _ from "lodash-es";
_.isEqual(1, 1);
import * as R from "ramda";
R.equals(1, 1);
"#,
    ];

    rule_tester.run("use_different_package", use_different_package, &valid_code);

    let invalid_code = vec![
        r#"
import * as _ from "lodash";
_.isEqual(1, 1);
"#,
        r#"
import _ from "lodash-es";
_.isEqual(1, 1);
"#,
        r#"
import _, { isEmpty } from "lodash-es";
_.isEqual(1, 1);
"#,
        r#"
import lodash, { isEmpty as lodashIsEmpty } from "lodash-es";
lodash.isEqual(1, 1);
lodashIsEmpty({});
"#,
        r#"
import ramda, { isEmpty as ramdaIsEmpty } from "ramda";
ramda.equals(1, 1);
ramdaIsEmpty({});
"#,
    ];

    rule_tester.run("use_namespace_import", use_namespace_import, &invalid_code);

    println!("All tests passed!");
}

fn use_different_package(cx: rustc_lint::Context) -> rustc_lint::LintResult {
    for item in cx.lookup_items_in_module("lodash-es") {
        if !cx.lookup_item_in_module("ramda").contains(item.name) {
            cx.struct_span_err(item.span, "Use different package for lodash-es and ramda");
        }
    }

    for item in cx.lookup_items_in_module("lodash-es") {
        if cx.lookup_item_in_module("R").contains(item.name) {
            cx.struct_span_err(item.span, "Use namespace import for lodash-es and ramda");
        }
    }

    Ok(())
}

fn use_namespace_import(cx: rustc_lint::Context) -> rustc_lint::LintResult {
    for item in cx.lookup_items_in_module("lodash-es") {
        if !cx.lookup_item_in_module("R").contains(item.name) {
            cx.struct_span_err(item.span, "Use different package for lodash-es and ramda");
        }
    }

    for item in cx.lookup_items_in_module("lodash-es") {
        if cx.lookup_item_in_module("_").contains(item.name) {
            cx.struct_span_err(item.span, "Use namespace import for lodash-es and ramda");
        }
    }

    Ok(())
}
```