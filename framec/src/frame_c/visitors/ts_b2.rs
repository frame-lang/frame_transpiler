// Experimental SWC codegen for TS MIR emission (B2 path)

use swc_common::{sync::Lrc, SourceMap, DUMMY_SP, SyntaxContext};
use swc_ecma_ast as ast;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

fn emit_stmts(stmts: Vec<ast::Stmt>) -> Option<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let mut buf = Vec::new();
    let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
    let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config::default(),
        comments: None,
        cm: cm.clone(),
        wr,
    };
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

pub(crate) fn b2_emit_transition(state: &str) -> Option<String> {
    // Build `this._frame_transition(new FrameCompartment("State", null, null, {}, {})); return;`
    let callee = ast::Expr::Member(ast::MemberExpr {
        span: DUMMY_SP,
        obj: ast::Expr::This(ast::ThisExpr { span: DUMMY_SP }).into(),
        prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new(
            "_frame_transition".into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        ))),
    });

    let new_fc = ast::Expr::New(ast::NewExpr {
        span: DUMMY_SP,
        callee: ast::Callee::Expr(ast::Expr::Ident(ast::Ident::new(
            "FrameCompartment".into(),
            DUMMY_SP,
            SyntaxContext::empty(),
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
    emit_stmts(stmts)
}

pub(crate) fn b2_emit_parent_forward(target: &str) -> Option<String> {
    // `this._nextCompartment = new FrameCompartment(target, null, null, {}, {}); this._nextCompartment.forwardEvent = __e; return;`
    // Build left target: this._nextCompartment
    let left_next_compartment_expr = ast::Expr::Member(ast::MemberExpr {
        span: DUMMY_SP,
        obj: ast::Expr::This(ast::ThisExpr { span: DUMMY_SP }).into(),
        prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new(
            "_nextCompartment".into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        ))),
    });
    let left_next_compartment: ast::AssignTarget = ast::AssignTarget::try_from(Box::new(left_next_compartment_expr))
        .unwrap_or_else(|_| ast::AssignTarget::Pat(ast::AssignTargetPat::Invalid(ast::Invalid { span: DUMMY_SP })));

    let assign = ast::Stmt::Expr(ast::ExprStmt {
        span: DUMMY_SP,
        expr: ast::Expr::Assign(ast::AssignExpr {
            span: DUMMY_SP,
            op: ast::AssignOp::Assign,
            left: left_next_compartment,
            right: ast::Expr::New(ast::NewExpr {
                span: DUMMY_SP,
                callee: ast::Callee::Expr(ast::Expr::Ident(ast::Ident::new(
                    "FrameCompartment".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                )).into()),
                args: Some(vec![
                    ast::ExprOrSpread { spread: None, expr: ast::Expr::from(ast::Lit::Str(ast::Str{ span: DUMMY_SP, value: target.into(), raw: None})).into() },
                    ast::ExprOrSpread { spread: None, expr: ast::Expr::Null(ast::Null { span: DUMMY_SP }).into() },
                    ast::ExprOrSpread { spread: None, expr: ast::Expr::Null(ast::Null { span: DUMMY_SP }).into() },
                    ast::ExprOrSpread { spread: None, expr: ast::Expr::Object(ast::ObjectLit { span: DUMMY_SP, props: vec![] }).into() },
                    ast::ExprOrSpread { spread: None, expr: ast::Expr::Object(ast::ObjectLit { span: DUMMY_SP, props: vec![] }).into() },
                ]),
                type_args: None,
            }).into(),
        }).into(),
    });
    // Build left target: this._nextCompartment.forwardEvent
    let left_forward_event_expr = ast::Expr::Member(ast::MemberExpr {
        span: DUMMY_SP,
        obj: ast::Expr::Member(ast::MemberExpr {
            span: DUMMY_SP,
            obj: ast::Expr::This(ast::ThisExpr { span: DUMMY_SP }).into(),
            prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new(
                "_nextCompartment".into(),
                DUMMY_SP,
                SyntaxContext::empty(),
            ))),
        }).into(),
        prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new(
            "forwardEvent".into(),
            DUMMY_SP,
            SyntaxContext::empty(),
        ))),
    });
    let left_forward_event: ast::AssignTarget = ast::AssignTarget::try_from(Box::new(left_forward_event_expr))
        .unwrap_or_else(|_| ast::AssignTarget::Pat(ast::AssignTargetPat::Invalid(ast::Invalid { span: DUMMY_SP })));

    let set_fwd = ast::Stmt::Expr(ast::ExprStmt {
        span: DUMMY_SP,
        expr: ast::Expr::Assign(ast::AssignExpr {
            span: DUMMY_SP,
            op: ast::AssignOp::Assign,
            left: left_forward_event,
            right: ast::Expr::Ident(ast::Ident::new("__e".into(), DUMMY_SP, SyntaxContext::empty())).into(),
        }).into(),
    });
    emit_stmts(vec![assign, set_fwd, ast::Stmt::Return(ast::ReturnStmt { span: DUMMY_SP, arg: None })])
}

pub(crate) fn b2_emit_stack_push() -> Option<String> {
    let push = ast::Stmt::Expr(ast::ExprStmt { span: DUMMY_SP, expr: ast::Expr::Call(ast::CallExpr {
        span: DUMMY_SP,
        callee: ast::Callee::Expr(ast::Expr::Member(ast::MemberExpr{
            span: DUMMY_SP,
            obj: ast::Expr::This(ast::ThisExpr{ span: DUMMY_SP }).into(),
            prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new("returnStack".into(), DUMMY_SP, SyntaxContext::empty()))),
        }).into()),
        args: vec![ast::ExprOrSpread{ spread: None, expr: ast::Expr::Object(ast::ObjectLit{ span: DUMMY_SP, props: vec![] }).into()}],
        type_args: None,
    }).into() });
    emit_stmts(vec![push, ast::Stmt::Return(ast::ReturnStmt{ span: DUMMY_SP, arg: None })])
}

pub(crate) fn b2_emit_stack_pop() -> Option<String> {
    let decl = ast::Stmt::Decl(ast::Decl::Var(Box::new(ast::VarDecl{
        span: DUMMY_SP,
        kind: ast::VarDeclKind::Const,
        declare: false,
        decls: vec![ast::VarDeclarator{
            span: DUMMY_SP,
            name: ast::Pat::Ident(ast::BindingIdent{ id: ast::Ident::new("__popped".into(), DUMMY_SP, SyntaxContext::empty()), type_ann: None }),
            init: Some(ast::Expr::Call(ast::CallExpr{
                span: DUMMY_SP,
                callee: ast::Callee::Expr(ast::Expr::Member(ast::MemberExpr{
                    span: DUMMY_SP,
                    obj: ast::Expr::Member(ast::MemberExpr{
                        span: DUMMY_SP,
                        obj: ast::Expr::This(ast::ThisExpr{ span: DUMMY_SP }).into(),
                        prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new("returnStack".into(), DUMMY_SP, SyntaxContext::empty()))),
                    }).into(),
                    prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new("pop".into(), DUMMY_SP, SyntaxContext::empty()))),
                }).into()),
                args: vec![],
                type_args: None,
            }).into()),
            definite: false,
        }],
    })));
    // Build left target: this.returnStack[this.returnStack.length - 1]
    let left_stack_index_expr = ast::Expr::Member(ast::MemberExpr{
        span: DUMMY_SP,
        obj: ast::Expr::Member(ast::MemberExpr{
            span: DUMMY_SP,
            obj: ast::Expr::This(ast::ThisExpr{ span: DUMMY_SP }).into(),
            prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new("returnStack".into(), DUMMY_SP, SyntaxContext::empty()))),
        }).into(),
        prop: ast::MemberProp::Computed(ast::ComputedPropName{ span: DUMMY_SP, expr: ast::Expr::Bin(ast::BinExpr{
            span: DUMMY_SP,
            op: ast::BinaryOp::Sub,
            left: ast::Expr::Member(ast::MemberExpr{
                span: DUMMY_SP,
                obj: ast::Expr::This(ast::ThisExpr{ span: DUMMY_SP }).into(),
                prop: ast::MemberProp::Ident(ast::IdentName::from(ast::Ident::new("returnStack".into(), DUMMY_SP, SyntaxContext::empty()))),
            }).into(),
            right: ast::Expr::Lit(ast::Lit::Num(ast::Number{ span: DUMMY_SP, value: 1.0 })).into(),
        }).into() })
    });
    let left_stack_index: ast::AssignTarget = ast::AssignTarget::try_from(Box::new(left_stack_index_expr))
        .unwrap_or_else(|_| ast::AssignTarget::Pat(ast::AssignTargetPat::Invalid(ast::Invalid { span: DUMMY_SP })));

    let assign = ast::Stmt::Expr(ast::ExprStmt{ span: DUMMY_SP, expr: ast::Expr::Assign(ast::AssignExpr{
        span: DUMMY_SP,
        op: ast::AssignOp::Assign,
        left: left_stack_index,
        right: ast::Expr::Ident(ast::Ident::new("__popped".into(), DUMMY_SP, SyntaxContext::empty())).into(),
    }).into() });
    emit_stmts(vec![decl, assign, ast::Stmt::Return(ast::ReturnStmt{ span: DUMMY_SP, arg: None })])
}

pub(crate) fn b2_emit_return() -> Option<String> {
    emit_stmts(vec![ast::Stmt::Return(ast::ReturnStmt{ span: DUMMY_SP, arg: None })])
}
