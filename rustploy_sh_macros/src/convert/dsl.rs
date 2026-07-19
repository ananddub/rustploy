use quote::quote;
use syn::ExprMethodCall;

pub fn convert_method_call(method_call: &ExprMethodCall) -> Result<proc_macro2::TokenStream, syn::Error> {
    let receiver_tokens = crate::convert::convert_expr(&method_call.receiver)?;
    let method_name = method_call.method.to_string();

    if method_name == "stdout" {
        let arg = method_call.args.first().ok_or_else(|| syn::Error::new_spanned(method_call, "stdout method needs one argument"))?;
        let arg_tokens = crate::convert::convert_expr(arg)?;
        return Ok(quote! {
            (#receiver_tokens).stdout(
                match #arg_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                    _ => panic!("stdout redirect target must be a string literal"),
                }
            )
        });
    }

    if method_name == "stderr" {
        let arg = method_call.args.first().ok_or_else(|| syn::Error::new_spanned(method_call, "stderr method needs one argument"))?;
        let arg_tokens = crate::convert::convert_expr(arg)?;
        return Ok(quote! {
            (#receiver_tokens).stderr(
                match #arg_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                    _ => panic!("stderr redirect target must be a string literal"),
                }
            )
        });
    }

    if method_name == "sudo" {
        return Ok(quote! {
            (#receiver_tokens).sudo()
        });
    }

    if method_name == "success" {
        return Ok(quote! {
            (#receiver_tokens).success()
        });
    }

    if method_name == "failure" {
        return Ok(quote! {
            (#receiver_tokens).failure()
        });
    }

    if method_name == "ok" {
        return Ok(quote! {
            (#receiver_tokens).ok()
        });
    }

    let expr = syn::Expr::MethodCall(method_call.clone());
    Ok(quote! {
        (crate::utils::exec::script::dsl::ShellIRWrapper(#expr).into_shell_ir())
    })
}
