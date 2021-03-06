//! lint on enum variants that are prefixed or suffixed by the same characters

use rustc::lint::*;
use syntax::ast::*;
use syntax::parse::token::InternedString;
use utils::span_help_and_lint;
use utils::{camel_case_from, camel_case_until};

/// **What it does:** Warns on enum variants that are prefixed or suffixed by the same characters
///
/// **Why is this bad?** Enum variant names should specify their variant, not the enum, too.
///
/// **Known problems:** None
///
/// **Example:** enum Cake { BlackForestCake, HummingbirdCake }
declare_lint! {
    pub ENUM_VARIANT_NAMES, Warn,
    "finds enums where all variants share a prefix/postfix"
}

pub struct EnumVariantNames;

impl LintPass for EnumVariantNames {
    fn get_lints(&self) -> LintArray {
        lint_array!(ENUM_VARIANT_NAMES)
    }
}

fn var2str(var: &Variant) -> InternedString {
    var.node.name.name.as_str()
}

// FIXME: waiting for https://github.com/rust-lang/rust/pull/31700
// fn partial_match(pre: &str, name: &str) -> usize {
//    // skip(1) to ensure that the prefix never takes the whole variant name
//     pre.chars().zip(name.chars().rev().skip(1).rev()).take_while(|&(l, r)| l == r).count()
// }
//
// fn partial_rmatch(post: &str, name: &str) -> usize {
//     // skip(1) to ensure that the postfix never takes the whole variant name
//     post.chars().rev().zip(name.chars().skip(1).rev()).take_while(|&(l, r)| l == r).count()
// }

fn partial_match(pre: &str, name: &str) -> usize {
    let mut name_iter = name.chars();
    let _ = name_iter.next_back(); // make sure the name is never fully matched
    pre.chars().zip(name_iter).take_while(|&(l, r)| l == r).count()
}

fn partial_rmatch(post: &str, name: &str) -> usize {
    let mut name_iter = name.chars();
    let _ = name_iter.next(); // make sure the name is never fully matched
    post.chars().rev().zip(name_iter.rev()).take_while(|&(l, r)| l == r).count()
}

impl EarlyLintPass for EnumVariantNames {
    // FIXME: #600
    #[allow(while_let_on_iterator)]
    fn check_item(&mut self, cx: &EarlyContext, item: &Item) {
        if let ItemKind::Enum(ref def, _) = item.node {
            if def.variants.len() < 2 {
                return;
            }
            let first = var2str(&def.variants[0]);
            let mut pre = &first[..camel_case_until(&*first)];
            let mut post = &first[camel_case_from(&*first)..];
            for var in &def.variants {
                let name = var2str(var);

                let pre_match = partial_match(pre, &name);
                pre = &pre[..pre_match];
                let pre_camel = camel_case_until(pre);
                pre = &pre[..pre_camel];
                while let Some((next, last)) = name[pre.len()..].chars().zip(pre.chars().rev()).next() {
                    if next.is_lowercase() {
                        let last = pre.len() - last.len_utf8();
                        let last_camel = camel_case_until(&pre[..last]);
                        pre = &pre[..last_camel];
                    } else {
                        break;
                    }
                }

                let post_match = partial_rmatch(post, &name);
                let post_end = post.len() - post_match;
                post = &post[post_end..];
                let post_camel = camel_case_from(post);
                post = &post[post_camel..];
            }
            let (what, value) = match (pre.is_empty(), post.is_empty()) {
                (true, true) => return,
                (false, _) => ("pre", pre),
                (true, false) => ("post", post),
            };
            span_help_and_lint(cx,
                               ENUM_VARIANT_NAMES,
                               item.span,
                               &format!("All variants have the same {}fix: `{}`", what, value),
                               &format!("remove the {}fixes and use full paths to \
                                         the variants instead of glob imports",
                                        what));
        }
    }
}
