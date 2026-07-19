extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod parser;
mod convert;

use parser::ShInput;
use convert::convert_stmt;

#[proc_macro]
pub fn sh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ShInput);
    let mut ir_constructions = Vec::new();

    let mut tracker = convert::scope::ScopeTracker::new();
    if let Err(err) = convert::scope::check_stmts(&input.stmts, &mut tracker) {
        return err.to_compile_error().into();
    }

    for stmt in input.stmts {
        match convert_stmt(&stmt) {
            Ok(tokens) => ir_constructions.push(tokens),
            Err(err) => return err.to_compile_error().into(),
        }
    }

    let expanded = quote! {
        ({
            use crate::utils::exec::script::dsl::IntoShellIRGeneric;
            let mut steps: Vec<crate::utils::exec::script::dsl::ShellIR> = Vec::new();
            #( steps.push(crate::utils::exec::script::dsl::ShellIRWrapper(#ir_constructions).into_shell_ir()); )*
            steps
        })
    };

    TokenStream::from(expanded)
}
