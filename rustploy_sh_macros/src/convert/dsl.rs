use quote::quote;
use syn::ExprMethodCall;

fn is_os_receiver(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Path(expr_path) => {
            expr_path.path.get_ident().map(|id| id == "os").unwrap_or(false)
        }
        syn::Expr::MethodCall(method_call) => {
            is_os_receiver(&method_call.receiver)
        }
        syn::Expr::Field(expr_field) => {
            is_os_receiver(&expr_field.base)
        }
        _ => false,
    }
}

pub fn convert_method_call(method_call: &ExprMethodCall) -> Result<proc_macro2::TokenStream, syn::Error> {
    let receiver = &*method_call.receiver;
    let method_name = method_call.method.to_string();
    let is_special_method = method_name == "stdout"
        || method_name == "stderr"
        || method_name == "sudo"
        || method_name == "success"
        || method_name == "failure"
        || method_name == "ok";

    let receiver_tokens = if is_special_method {
        if is_os_receiver(receiver) {
            quote! {
                (crate::utils::exec::script::dsl::IntoShellIRGeneric::into_shell_ir(
                    crate::utils::exec::script::dsl::ShellIRWrapper(#receiver)
                ))
            }
        } else {
            crate::convert::convert_expr(receiver)?
        }
    } else {
        if is_os_receiver(receiver) {
            quote! { #receiver }
        } else {
            crate::convert::convert_expr(receiver)?
        }
    };

    if method_name == "stdout" {
        if method_call.args.is_empty() {
            return Ok(quote! {
                (match #receiver_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                        crate::utils::exec::script::dsl::ShellIR::Expr(
                            crate::utils::exec::script::dsl::Expr::Variable(format!("{}_stdout", v))
                        )
                    }
                    _ => panic!("stdout() getter can only be called on a variable reference"),
                })
            });
        }
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
        if method_call.args.is_empty() {
            return Ok(quote! {
                (match #receiver_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                        crate::utils::exec::script::dsl::ShellIR::Expr(
                            crate::utils::exec::script::dsl::Expr::Variable(format!("{}_stderr", v))
                        )
                    }
                    _ => panic!("stderr() getter can only be called on a variable reference"),
                })
            });
        }
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

    let mut args_tokens = Vec::new();
    for arg in &method_call.args {
        if let syn::Expr::Path(expr_path) = arg {
            let is_os = expr_path.path.get_ident().map(|id| id == "os").unwrap_or(false);
            if is_os {
                args_tokens.push(quote! { #arg });
            } else {
                args_tokens.push(crate::convert::convert_expr(arg)?);
            }
        } else if let syn::Expr::Macro(_) = arg {
            args_tokens.push(crate::convert::convert_expr(arg)?);
        } else {
            args_tokens.push(quote! { #arg });
        }
    }
    let method_ident = &method_call.method;
    Ok(quote! {
        (crate::utils::exec::script::dsl::ShellIRWrapper(
            (#receiver_tokens).#method_ident( #( #args_tokens ),* )
        ).into_shell_ir())
    })
}
