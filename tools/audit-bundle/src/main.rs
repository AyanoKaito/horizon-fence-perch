// Read-only AST audit: remove exactly the declared additions in memory, then
// compare the remaining Rust syntax with pristine source. Never writes source.
use quote::ToTokens;
use std::{error::Error, fs, path::Path};
use syn::{
    Expr, ExprClosure, File, ImplItemFn, Item, ItemFn, Stmt,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// A cancellable HEAD hook: `if let Break(r) = { helper; call } { return r; }`.
struct Head {
    method: &'static str,
    helper: ItemFn,
    call: Expr,
    seen: usize,
}

/// A RETURN hook: every `return v` and the tail `v` become `({ helper name })(v, args..)`.
struct Return {
    method: &'static str,
    callee: Expr,
    args: Vec<String>,
    unwrapped: usize,
    expected: usize,
}

struct Declared {
    file: &'static str,
    modules: &'static [(&'static str, &'static str)],
    heads: &'static [(&'static str, &'static str, &'static str)],
    returns: &'static [(&'static str, &'static str, &'static str, &'static [&'static str])],
}

const DECLARED: &[Declared] = &[
    Declared {
        file: "crates/horizon-client/src/client_lib.rs",
        modules: &[
            ("fence_perch_core", "core.rs"),
            ("fence_perch_query", "query.rs"),
            ("fence_perch_validation", "validator.rs"),
        ],
        heads: &[],
        returns: &[],
    },
    Declared {
        file: "crates/horizon-client/src/motion.rs",
        modules: &[("fence_perch", "runtime.rs")],
        heads: &[
            ("collide", "collide.rs", "collide(&mut *self, movement)"),
            ("no_player_collision", "clear.rs", "clear(&mut *self, bounds)"),
            (
                "set_on_ground_with_movement",
                "ground.rs",
                "ground(&mut *self, below, horizontal, movement)",
            ),
        ],
        returns: &[],
    },
    Declared {
        file: "crates/horizon-client/src/lan/hub.rs",
        modules: &[("fence_perch_server", "server.rs")],
        heads: &[],
        returns: &[("collide", "result.rs", "adapt_result", &["self", "i", "to", "ctx"])],
    },
];

fn tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

fn item_text(item: &Item) -> String {
    prettyplease::unparse(&File { shebang: None, attrs: Vec::new(), items: vec![item.clone()] })
}

fn stmt_text(stmt: &Stmt) -> String {
    let body = syn::Block { brace_token: Default::default(), stmts: vec![stmt.clone()] };
    let function: ItemFn = syn::parse_quote!(fn audit_head() #body);
    item_text(&Item::Fn(function))
}

fn expr_text(expr: &Expr) -> String {
    stmt_text(&Stmt::Expr(expr.clone(), Some(Default::default())))
}

fn parse(path: &Path) -> Result<File> {
    Ok(syn::parse_file(&fs::read_to_string(path)?)?)
}

struct RemoveHooks {
    heads: Vec<Head>,
    returns: Vec<Return>,
    error: Option<String>,
}

impl RemoveHooks {
    fn strip_head(&mut self, method: &mut ImplItemFn) {
        for hook in &mut self.heads {
            if method.sig.ident != hook.method {
                continue;
            }
            let helper = &hook.helper;
            let call = &hook.call;
            let expected: Stmt = syn::parse_quote! {
                if let ::core::ops::ControlFlow::Break(__horizon_mixins_return) = {
                    #helper
                    #call
                } { return __horizon_mixins_return; }
            };
            // Same-named methods in other impls stay untouched and are compared as-is.
            if method.block.stmts.first().is_some_and(|s| stmt_text(s) == stmt_text(&expected)) {
                method.block.stmts.remove(0);
                hook.seen += 1;
            }
        }
    }
}

/// Count `return` expressions outside closures (RETURN hooks do not enter closures).
struct CountReturns(usize);

impl<'a> Visit<'a> for CountReturns {
    fn visit_expr_return(&mut self, node: &'a syn::ExprReturn) {
        self.0 += 1;
        visit::visit_expr_return(self, node);
    }
    fn visit_expr_closure(&mut self, _: &'a ExprClosure) {}
}

struct Unwrap<'h> {
    hook: &'h mut Return,
    error: Option<String>,
}

impl VisitMut for Unwrap<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        let Expr::Call(call) = expr else { return };
        // Canonical printing: the transformer's reprint may add trailing commas.
        if expr_text(&call.func) != expr_text(&self.hook.callee) {
            return;
        }
        let args: Vec<_> = call.args.iter().collect();
        let tail: Vec<String> = args.iter().skip(1).map(|a| tokens(*a)).collect();
        if args.len() != self.hook.args.len() + 1 || tail != self.hook.args {
            self.error = Some(format!("Unexpected RETURN arguments in {}", self.hook.method));
            return;
        }
        let value = args[0].clone();
        *expr = value;
        self.hook.unwrapped += 1;
    }
    fn visit_expr_closure_mut(&mut self, _: &mut ExprClosure) {}
}

impl VisitMut for RemoveHooks {
    fn visit_impl_item_fn_mut(&mut self, method: &mut ImplItemFn) {
        self.strip_head(method);
        for hook in &mut self.returns {
            if method.sig.ident != hook.method {
                continue;
            }
            let mut unwrap = Unwrap { hook, error: None };
            unwrap.visit_block_mut(&mut method.block);
            if let Some(error) = unwrap.error {
                self.error = Some(error);
            }
        }
        visit_mut::visit_impl_item_fn_mut(self, method);
    }
}

fn remove_module(generated: &mut File, name: &str, source: &File) -> Result<()> {
    let found: Vec<_> = generated
        .items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| matches!(item, Item::Mod(m) if m.ident == name).then_some(i))
        .collect();
    if found.len() != 1 {
        return Err(format!("Expected exactly one module {name}").into());
    }
    let Item::Mod(module) = generated.items.remove(found[0]) else { unreachable!() };
    let Some((_, items)) = module.content else {
        return Err(format!("Expected inline module {name}").into());
    };
    let mismatch = items.iter().zip(&source.items).position(|(a, b)| item_text(a) != item_text(b));
    if items.len() != source.items.len()
        || mismatch.is_some()
        || tokens(&module.vis) != "pub (crate)"
        || !module.attrs.is_empty()
    {
        return Err(format!("Module {name} differs from bundle source (item {mismatch:?})").into());
    }
    Ok(())
}

fn audit(source: &Path, stage: &Path, bundle: &Path, declared: &Declared) -> Result<()> {
    let original = parse(&source.join(declared.file))?;
    let mut generated = parse(&stage.join(declared.file))?;
    for (name, file) in declared.modules {
        remove_module(&mut generated, name, &parse(&bundle.join(file))?)?;
    }
    let mut heads = Vec::new();
    for (method, helper, call) in declared.heads {
        heads.push(Head {
            method,
            helper: syn::parse_str(&fs::read_to_string(bundle.join(helper))?)?,
            call: syn::parse_str(call)?,
            seen: 0,
        });
    }
    let mut returns = Vec::new();
    for (method, helper, name, args) in declared.returns {
        let helper: ItemFn = syn::parse_str(&fs::read_to_string(bundle.join(helper))?)?;
        let name: syn::Ident = syn::parse_str(name)?;
        let callee: Expr = syn::parse_quote!(({ #helper #name }));
        // Expected wraps: every return outside closures plus the tail expression.
        let mut expected = 0;
        let mut found = 0;
        visit::visit_file(
            &mut MethodReturns { method, expected: &mut expected, found: &mut found },
            &original,
        );
        if found != 1 {
            return Err(format!("Expected one pristine method {method}").into());
        }
        returns.push(Return {
            method,
            callee,
            args: args.iter().map(|a| a.to_string()).collect(),
            unwrapped: 0,
            expected,
        });
    }
    let mut visitor = RemoveHooks { heads, returns, error: None };
    visitor.visit_file_mut(&mut generated);
    if let Some(error) = visitor.error {
        return Err(error.into());
    }
    if let Some(hook) = visitor.heads.iter().find(|h| h.seen != 1) {
        return Err(format!("HEAD hook {} matched {} times", hook.method, hook.seen).into());
    }
    if let Some(hook) = visitor.returns.iter().find(|h| h.unwrapped != h.expected) {
        return Err(format!(
            "RETURN hook {} unwrapped {} of {} exits",
            hook.method, hook.unwrapped, hook.expected
        )
        .into());
    }
    // Canonical printing normalizes insignificant layout and drops comments.
    if prettyplease::unparse(&original) != prettyplease::unparse(&generated) {
        return Err(format!("Original AST was not preserved in {}", declared.file).into());
    }
    println!(
        "AST preserved: {} ({} modules, {} HEAD, {} RETURN exits)",
        declared.file,
        declared.modules.len(),
        visitor.heads.len(),
        visitor.returns.iter().map(|h| h.unwrapped).sum::<usize>()
    );
    Ok(())
}

/// Expected RETURN exits of the pristine target: returns outside closures plus a tail.
struct MethodReturns<'a> {
    method: &'a str,
    expected: &'a mut usize,
    found: &'a mut usize,
}

impl<'v> Visit<'v> for MethodReturns<'_> {
    fn visit_impl_item_fn(&mut self, node: &'v ImplItemFn) {
        if node.sig.ident == self.method && !matches!(node.sig.output, syn::ReturnType::Default) {
            let mut count = CountReturns(0);
            count.visit_block(&node.block);
            let tail = matches!(node.block.stmts.last(), Some(Stmt::Expr(_, None)));
            *self.expected = count.0 + usize::from(tail);
            *self.found += 1;
        }
        visit::visit_impl_item_fn(self, node);
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err("Usage: audit-bundle SOURCE STAGE BUNDLE".into());
    }
    let (source, stage, bundle) = (Path::new(&args[0]), Path::new(&args[1]), Path::new(&args[2]));
    for declared in DECLARED {
        audit(source, stage, bundle, declared)?;
    }
    println!("Verified five bundle modules, three HEAD hooks and one RETURN hook.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn return_hook(expected: usize) -> Return {
        Return {
            method: "action",
            callee: syn::parse_quote!(({ fn hook(v: u8, a: u8) -> u8 { v } hook })),
            args: vec!["a".into()],
            unwrapped: 0,
            expected,
        }
    }

    fn strip(source: &str, hook: Return) -> (File, RemoveHooks) {
        let mut staged = syn::parse_file(source).unwrap();
        let mut check = RemoveHooks { heads: vec![], returns: vec![hook], error: None };
        check.visit_file_mut(&mut staged);
        (staged, check)
    }

    #[test]
    fn unwraps_every_return_and_the_tail() {
        let original = syn::parse_file(
            "impl S { fn action(a: u8) -> u8 { if a > 1 { return 1; } let f = || { return 3; }; 2 } }",
        )
        .unwrap();
        let (staged, check) = strip(
            "impl S { fn action(a: u8) -> u8 { if a > 1 { return ({ fn hook(v: u8, a: u8) -> u8 { v } hook })(1, a); } let f = || { return 3; }; ({ fn hook(v: u8, a: u8) -> u8 { v } hook })(2, a) } }",
            return_hook(2),
        );
        assert!(check.error.is_none());
        assert_eq!(check.returns[0].unwrapped, 2);
        assert_eq!(prettyplease::unparse(&original), prettyplease::unparse(&staged));
    }

    #[test]
    fn rejects_an_altered_return_helper() {
        let (_, check) = strip(
            "impl S { fn action(a: u8) -> u8 { ({ fn hook(v: u8, a: u8) -> u8 { a } hook })(2, a) } }",
            return_hook(1),
        );
        assert_eq!(check.returns[0].unwrapped, 0);
    }

    #[test]
    fn rejects_altered_return_arguments() {
        let (_, check) = strip(
            "impl S { fn action(a: u8) -> u8 { ({ fn hook(v: u8, a: u8) -> u8 { v } hook })(2, 7) } }",
            return_hook(1),
        );
        assert!(check.error.is_some());
    }

    #[test]
    fn counts_pristine_exits_outside_closures() {
        let original = syn::parse_file(
            "impl S { fn action(a: u8) -> u8 { if a > 1 { return 1; } let f = || { return 3; }; 2 } }",
        )
        .unwrap();
        let (mut expected, mut found) = (0, 0);
        visit::visit_file(
            &mut MethodReturns { method: "action", expected: &mut expected, found: &mut found },
            &original,
        );
        assert_eq!((expected, found), (2, 1));
    }

    #[test]
    fn strips_only_a_matching_cancellable_head() {
        let original = syn::parse_file("impl S { fn action() { let kept = 42; } }").unwrap();
        let mut staged = syn::parse_file(
            "impl S { fn action() { if let ::core::ops::ControlFlow::Break(__horizon_mixins_return) = { fn hook() {} hook() } { return __horizon_mixins_return; } let kept = 42; } }",
        )
        .unwrap();
        let mut check = RemoveHooks {
            heads: vec![Head {
                method: "action",
                helper: syn::parse_str("fn hook() {}").unwrap(),
                call: syn::parse_str("hook()").unwrap(),
                seen: 0,
            }],
            returns: vec![],
            error: None,
        };
        check.visit_file_mut(&mut staged);
        assert_eq!(check.heads[0].seen, 1);
        assert_eq!(prettyplease::unparse(&original), prettyplease::unparse(&staged));
    }
}
