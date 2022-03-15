use super::TransformVisitor;
use std::path::PathBuf;
use swc_ecma_transforms_testing::{test, test_fixture};
use swc_ecmascript::parser::{EsConfig, Syntax};
use swc_plugin::ast::*;
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

fn transformer() -> impl Fold {
    as_folder(TransformVisitor)
}

#[fixture("fixture/**/input.js")]
fn replacer_console_log(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(syntax(), &|_tr| transformer(), &input, &output);
}
