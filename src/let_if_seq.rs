use rustc::lint::*;
use rustc_front::hir::*;
use syntax::codemap;
use utils::{snippet, span_lint_and_then};

/// **What it does:** This lint checks for variable declarations immediatly followed by a
/// conditionnal affectation.
///
/// **Why is this bad?** This is not idiomatic Rust.
///
/// **Known problems:** None.
///
/// **Example:** 
/// ```rust,ignore
/// let foo;
///
/// if bar() {
///     foo = 42;
/// } else {
///     foo = 0;
/// }
///
/// let mut baz = None;
///
/// if bar() {
///     baz = Some(42);
/// }
/// ```
///
/// should be writen
///
/// ```rust,ignore
/// let foo = if bar() {
///     42;
/// } else {
///     0;
/// };
///
/// let baz = if bar() {
///     Some(42);
/// } else {
///     None
/// };
/// ```
declare_lint! {
    pub USELESS_LET_IF_SEQ,
    Warn,
    "TODO"
}

#[derive(Copy,Clone)]
pub struct LetIfSeq;

impl LintPass for LetIfSeq {
    fn get_lints(&self) -> LintArray {
        lint_array!(USELESS_LET_IF_SEQ)
    }
}

impl LateLintPass for LetIfSeq {
    fn check_block(&mut self, cx: &LateContext, block: &Block) {
        let mut it = block.stmts.iter().peekable();
        while let Some(ref stmt) = it.next() {
            if_let_chain! {[
                let StmtDecl(ref decl, _) = stmt.node,
                let DeclLocal(ref decl) = decl.node,
                let PatKind::Ident(mode, ref name, None) = decl.pat.node,
                let Some(expr) = it.peek(),
                let StmtExpr(ref if_, _) = expr.node,
                let ExprIf(ref cond, ref then, ref else_) = if_.node,
                let Some(value) = check_assign(name.node, then),
            ], {
                let span = codemap::mk_sp(stmt.span.lo, if_.span.hi);

                let default = if let Some(ref else_) = *else_ {
                    if let ExprBlock(ref else_) = else_.node {
                        if let Some(default) = check_assign(name.node, else_) {
                            default
                        } else if let Some(ref default) = decl.init {
                            &*default
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                } else if let Some(ref default) = decl.init {
                    &*default
                } else {
                    return;
                };

                let mutability = match mode {
                    BindByRef(MutMutable) | BindByValue(MutMutable) => "<mut> ",
                    _ => "",
                };

                // FIXME: this should not suggest `mut` if we can detect that the variable is not
                // use mutably after the `if`

                let sug = format!(
                    "let {mut}{name} = if {cond} {{{then} {value} }} else {{{else} {default} }};",
                    mut=mutability,
                    name=name.node.name,
                    cond=snippet(cx, cond.span, "_"),
                    then=if then.stmts.len() > 1 { " ..;" } else { "" },
                    else=if else_.is_some() { " ..;" } else { "" },
                    value=snippet(cx, value.span, "<value>"),
                    default=snippet(cx, default.span, "<default>"),
                );
                span_lint_and_then(cx,
                                   USELESS_LET_IF_SEQ,
                                   span,
                                   "`let foo;`/`if .. { foo =  ..; }` sequence detected",
                                   |db| {
                                       db.span_suggestion(span,
                                                          "it is more idiomatic to write",
                                                          sug);
                                       if !mutability.is_empty() {
                                           db.note("you might not need `mut` at all");
                                       }
                                   });
            }}
        }
    }
}

fn check_assign(name: Ident, block: &Block) -> Option<&Expr> {
    if_let_chain! {[
        let Some(expr) = block.stmts.iter().last(),
        let StmtSemi(ref expr, _) = expr.node,
        let ExprAssign(ref var, ref value) = expr.node,
        let ExprPath(None, ref path) = var.node,
        path.segments.len() == 1,
        name.name.as_str() == path.segments[0].identifier.name.as_str(),
    ], {
        return Some(value);
    }}

    None
}
