// Experimental SWC codegen for TS MIR emission (B2 path)
// This module is compiled only when the `ts_b2_codegen` feature is enabled.

#![cfg(feature = "ts_b2_codegen")]

use swc_common::{SourceMap, DUMMY_SP};
use swc_ecma_ast as ast;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

pub(crate) fn b2_emit_transition(state: &str) -> Option<String> {
    let cm: std::sync::Arc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
    let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config { minify: false },
        comments: None,
        cm: cm.clone(),
        wr,
    };

    // Build `this._frame_transition(new FrameCompartment("State", null, null, {}, {})); return;`
    let callee = ast::Expr::Member(ast::MemberExpr {
        span: DUMMY_SP,
        obj: ast::Expr::This(ast::ThisExpr { span: DUMMY_SP }).into(),
        prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new(
            "_frame_transition".into(),
            DUMMY_SP,
        ))),
    });

    let new_fc = ast::Expr::New(ast::NewExpr {
        span: DUMMY_SP,
        callee: ast::Callee::Expr(ast::Expr::Ident(ast::Ident::new(
            "FrameCompartment".into(),
            DUMMY_SP,
        ))
        .into()),
        args: Some(vec![
            ast::ExprOrSpread {
                spread: None,
                expr: ast::Expr::from(ast::Lit::Str(ast::Str {
                    span: DUMMY_SP,
                    value: state.into(),
                    raw: None,
                }))
                .into(),
            },
            ast::ExprOrSpread { spread: None, expr: ast::Expr::Null(ast::Null { span: DUMMY_SP }).into() },
            ast::ExprOrSpread { spread: None, expr: ast::Expr::Null(ast::Null { span: DUMMY_SP }).into() },
            ast::ExprOrSpread { spread: None, expr: ast::Expr::Object(ast::ObjectLit { span: DUMMY_SP, props: vec![] }).into() },
            ast::ExprOrSpread { spread: None, expr: ast::Expr::Object(ast::ObjectLit { span: DUMMY_SP, props: vec![] }).into() },
        ]),
        type_args: None,
    });

    let call = ast::Expr::Call(ast::CallExpr {
        span: DUMMY_SP,
        callee: ast::Callee::Expr(callee.into()),
        args: vec![ast::ExprOrSpread { spread: None, expr: new_fc.into() }],
        type_args: None,
    });

    let stmts: Vec<ast::Stmt> = vec![
        ast::Stmt::Expr(ast::ExprStmt { span: DUMMY_SP, expr: call.into() }),
        ast::Stmt::Return(ast::ReturnStmt { span: DUMMY_SP, arg: None }),
    ];
    let program = ast::Script { span: DUMMY_SP, body: stmts, shebang: None };
    if emitter.emit_script(&program).is_err() {
        return None;
    }
    let mut out = String::from_utf8(buf).ok()?;
    if !out.ends_with('\n') {
        out.push('\n');
    }
    Some(out)
}

