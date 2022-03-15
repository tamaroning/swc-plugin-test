use swc_plugin::syntax_pos::DUMMY_SP;
use swc_plugin::{ast::*, plugin_transform};
#[cfg(test)]
mod test;

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(MemberExpr { obj, prop, .. }) = &**expr {
                if let MemberProp::Ident(ident) = prop {
                    if ident.sym == *"log" {
                        if let Expr::Ident(ident) = &**obj {
                            if ident.sym == *"console" {
                                call.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    has_escape: false,
                                    kind: StrKind::default(),
                                    value: JsWord::from("Hello World"),
                                })));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _plugin_config: String, _context: String) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
