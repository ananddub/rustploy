use std::collections::HashSet;
use syn::{Expr, Pat, Stmt};

pub struct ScopeTracker {
    scopes: Vec<HashSet<String>>,
}

impl ScopeTracker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashSet::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String) {
        if let Some(current) = self.scopes.last_mut() {
            current.insert(name);
        }
    }

    pub fn check(&self, name: &str, span: proc_macro2::Span) -> Result<(), syn::Error> {
        for scope in self.scopes.iter().rev() {
            if scope.contains(name) {
                return Ok(());
            }
        }

        let mut best_match = None;
        let mut min_dist = usize::MAX;
        for scope in &self.scopes {
            for var in scope {
                let dist = levenshtein_distance(name, var);
                if dist < min_dist && dist <= 2 {
                    min_dist = dist;
                    best_match = Some(var.clone());
                }
            }
        }

        if let Some(suggestion) = best_match {
            Err(syn::Error::new(span, format!("Undefined variable '{}'. Did you mean '{}'?", name, suggestion)))
        } else {
            Err(syn::Error::new(span, format!("Undefined variable '{}'", name)))
        }
    }
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();
    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1],
                    std::cmp::min(dp[i - 1][j], dp[i][j - 1])
                );
            }
        }
    }
    dp[len_a][len_b]
}

pub fn check_stmts(stmts: &[Stmt], tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    for stmt in stmts {
        match stmt {
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    check_expr(&init.expr, tracker)?;
                }
                let name = match &local.pat {
                    Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                    _ => return Err(syn::Error::new_spanned(&local.pat, "Expected variable name")),
                };
                tracker.declare(name);
            }
            Stmt::Expr(expr, _) => {
                if let Expr::Path(_) = expr {
                    // Skip checking standalone path statement (treated as outer Rust variable reference)
                } else {
                    check_expr(expr, tracker)?;
                }
            }
            Stmt::Macro(stmt_macro) => {
                let macro_name = stmt_macro.mac.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
                if macro_name != "rust" {
                    check_macro(&stmt_macro.mac, tracker)?;
                }
            }
            Stmt::Item(_) => {}
        }
    }
    Ok(())
}

fn check_expr(expr: &Expr, tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    match expr {
        Expr::Path(expr_path) => {
            if let Some(ident) = expr_path.path.get_ident() {
                tracker.check(&ident.to_string(), ident.span())?;
            }
        }
        Expr::Call(expr_call) => {
            for arg in &expr_call.args {
                check_expr(arg, tracker)?;
            }
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_name == "stdout" || method_name == "stderr" || method_name == "sudo" || method_name == "ok" {
                check_expr(&method_call.receiver, tracker)?;
                for arg in &method_call.args {
                    check_expr(arg, tracker)?;
                }
            }
        }
        Expr::Macro(expr_macro) => {
            let macro_name = expr_macro.mac.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            if macro_name == "rust" {
                return Ok(());
            }
            check_macro(&expr_macro.mac, tracker)?;
        }
        Expr::Array(expr_array) => {
            for elem in &expr_array.elems {
                check_expr(elem, tracker)?;
            }
        }
        Expr::If(expr_if) => {
            check_expr(&expr_if.cond, tracker)?;
            tracker.push_scope();
            check_stmts(&expr_if.then_branch.stmts, tracker)?;
            tracker.pop_scope();
            if let Some((_, else_expr)) = &expr_if.else_branch {
                tracker.push_scope();
                check_expr(else_expr, tracker)?;
                tracker.pop_scope();
            }
        }
        Expr::Block(expr_block) => {
            tracker.push_scope();
            check_stmts(&expr_block.block.stmts, tracker)?;
            tracker.pop_scope();
        }
        Expr::ForLoop(expr_for) => {
            check_expr(&expr_for.expr, tracker)?;
            tracker.push_scope();
            let var_name = match &*expr_for.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => return Err(syn::Error::new_spanned(&expr_for.pat, "Expected identifier for loop variable")),
            };
            tracker.declare(var_name);
            check_stmts(&expr_for.body.stmts, tracker)?;
            tracker.pop_scope();
        }
        Expr::Closure(closure) => {
            tracker.push_scope();
            for input in &closure.inputs {
                if let Pat::Ident(pat_ident) = input {
                    tracker.declare(pat_ident.ident.to_string());
                }
            }
            check_expr(&closure.body, tracker)?;
            tracker.pop_scope();
        }
        Expr::Lit(_) => {}
        _ => {}
    }
    Ok(())
}

fn check_macro(mac: &syn::Macro, tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    if let Ok(exprs) = mac.parse_body_with(parser) {
        for expr in exprs {
            check_expr(&expr, tracker)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Stmt};

    #[test]
    fn test_scope_validation_success() {
        let stmts: Vec<Stmt> = parse_quote! {
            let config = "foo";
            cmd("echo", config);
        };
        let mut tracker = ScopeTracker::new();
        assert!(check_stmts(&stmts, &mut tracker).is_ok());
    }

    #[test]
    fn test_scope_validation_undefined_variable() {
        let stmts: Vec<Stmt> = parse_quote! {
            let config = "foo";
            cmd("echo", confg);
        };
        let mut tracker = ScopeTracker::new();
        let res = check_stmts(&stmts, &mut tracker);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Undefined variable 'confg'. Did you mean 'config'?");
    }

    #[test]
    fn test_scope_validation_out_of_scope() {
        let stmts: Vec<Stmt> = parse_quote! {
            if true {
                let port = 8080;
            }
            cmd("echo", port);
        };
        let mut tracker = ScopeTracker::new();
        let res = check_stmts(&stmts, &mut tracker);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Undefined variable 'port'");
    }
}
