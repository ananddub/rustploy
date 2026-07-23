use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, FnArg, GenericArgument, ImplItem, Item, ItemFn, ItemImpl, ItemMod, LitStr, PatType,
    PathArguments, ReturnType, Type, parse_macro_input, spanned::Spanned,
};

const METHODS: &[&str] = &["get", "post", "put", "delete", "patch", "options", "head"];

/// Declares an Axum controller whose receiver methods can carry route markers.
///
/// Supported markers are `#[get]`, `#[post]`, `#[put]`, `#[delete]`,
/// `#[patch]`, `#[options]`, and `#[head]`, each with an optional path.
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_path = parse_macro_input!(attr as LitStr);
    let item = parse_macro_input!(item as Item);

    let expanded = match item {
        Item::Impl(item_impl) => expand_controller(base_path, item_impl),
        Item::Mod(item_mod) => expand_controller_module(base_path, item_mod),
        other => Err(syn::Error::new_spanned(
            other,
            "#[controller] must be placed on an inherent impl or inline module",
        )),
    };

    match expanded {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

macro_rules! route_attribute {
    ($name:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            standalone_route(stringify!($name), attr, item)
        }
    };
}

route_attribute!(get);
route_attribute!(post);
route_attribute!(put);
route_attribute!(delete);
route_attribute!(patch);
route_attribute!(options);
route_attribute!(head);

fn standalone_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let function = parse_macro_input!(item as ItemFn);
    match expand_standalone_route(method, path, function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_standalone_route(
    method: &str,
    path: LitStr,
    function: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "standalone route handlers must be async",
        ));
    }
    if !function.sig.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "generic standalone route handlers are not supported",
        ));
    }

    let handler = &function.sig.ident;
    let operation_id = format!("{handler}");
    let method = format_ident!("{method}");
    let method_name = method.to_string().to_ascii_uppercase();
    let factory = format_ident!("__auto_route_factory_{}", handler);
    let path = LitStr::new(&join_paths("", &path.value()), path.span());
    let tag = openapi_tag(&path.value());
    let argument_types = function
        .sig
        .inputs
        .iter()
        .map(|argument| match argument {
            FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "standalone route handlers cannot take self",
            )),
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let params = infer_params(&path.value(), &argument_types);
    let params = param_descriptor_tokens(&params);
    let request_schema =
        request_schema_descriptor_tokens(infer_request_body(&argument_types).as_ref());
    let response_schema =
        response_schema_descriptor_tokens(infer_response_body(&function.sig.output).as_ref());

    Ok(quote! {
        #function

        #[doc(hidden)]
        fn #factory<'a>(
            _container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                Ok(::auto_route::__private::axum::Router::new().route(
                    #path,
                    ::auto_route::__private::axum::routing::#method(#handler),
                ))
            })
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(#factory)
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::OpenApiRouteDescriptor::new(
                #method_name,
                #path,
                #operation_id,
                #tag,
                #params,
                #request_schema,
                #response_schema,
            )
        }
    })
}

struct Route {
    method: syn::Ident,
    handler: syn::Ident,
    path: LitStr,
    argument_types: Vec<Type>,
    request_body: Option<RequestBody>,
    response_body: Option<ResponseBody>,
    params: Vec<OpenApiParam>,
}

struct ModuleRoute {
    method: syn::Ident,
    handler: syn::Ident,
    path: LitStr,
    request_body: Option<RequestBody>,
    response_body: Option<ResponseBody>,
    params: Vec<OpenApiParam>,
}

#[derive(Clone)]
struct RequestBody {
    ty: Type,
    content: RequestContent,
}

#[derive(Clone, Copy)]
enum RequestContent {
    Json,
    Form,
}

#[derive(Clone)]
enum ResponseBody {
    Json(Type),
    Sse,
}

struct OpenApiParam {
    name: String,
    ty: Type,
    source: ParamSource,
}

#[derive(Clone, Copy)]
enum ParamSource {
    Path,
    Query,
}

fn expand_controller_module(
    base_path: LitStr,
    mut item_mod: ItemMod,
) -> syn::Result<proc_macro2::TokenStream> {
    let module_ident = item_mod.ident.clone();
    let Some((_, items)) = &mut item_mod.content else {
        return Err(syn::Error::new_spanned(
            &item_mod,
            "#[controller] requires an inline module: `mod name { ... }`",
        ));
    };
    let mut routes = Vec::new();

    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        let mut route_attributes = Vec::new();
        let mut retained = Vec::new();
        for attribute in std::mem::take(&mut function.attrs) {
            if let Some(method) = route_method(&attribute) {
                route_attributes.push((attribute, method));
            } else {
                retained.push(attribute);
            }
        }
        function.attrs = retained;

        if route_attributes.len() > 1 {
            return Err(syn::Error::new_spanned(
                &function.sig,
                "a module route function can have only one route attribute",
            ));
        }
        let Some((attribute, method)) = route_attributes.pop() else {
            continue;
        };
        validate_route_function(function, "module route functions")?;
        let argument_types = route_argument_types(function.sig.inputs.iter())?;
        let route_path = marker_path(&attribute)?;
        routes.push(ModuleRoute {
            method,
            handler: function.sig.ident.clone(),
            path: LitStr::new(
                &join_paths(&base_path.value(), &route_path),
                attribute.span(),
            ),
            request_body: infer_request_body(&argument_types),
            response_body: infer_response_body(&function.sig.output),
            params: infer_params(
                &join_paths(&base_path.value(), &route_path),
                &argument_types,
            ),
        });
    }

    if routes.is_empty() {
        return Err(syn::Error::new_spanned(
            &module_ident,
            "controller module contains no route functions",
        ));
    }

    let registrations = routes.iter().map(|route| {
        let method = &route.method;
        let handler = &route.handler;
        let path = &route.path;
        quote! {
            router = router.route(
                #path,
                ::auto_route::__private::axum::routing::#method(#handler),
            );
        }
    });
    let generated: Item = syn::parse2(quote! {
        #[doc(hidden)]
        fn __auto_route_factory_module<'a>(
            _container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let mut router = ::auto_route::__private::axum::Router::new();
                #(#registrations)*
                Ok(router)
            })
        }
    })?;
    let submission: Item = syn::parse2(quote! {
        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(__auto_route_factory_module)
        }
    })?;
    let openapi_submissions = routes
        .iter()
        .map(|route| {
            let method = route.method.to_string().to_ascii_uppercase();
            let path = &route.path;
            let operation_id = format!("{}::{}", module_ident, route.handler);
            let tag = openapi_tag(&path.value());
            let params = param_descriptor_tokens(&route.params);
            let request_schema = request_schema_descriptor_tokens(route.request_body.as_ref());
            let response_schema = response_schema_descriptor_tokens(route.response_body.as_ref());
            quote! {
                ::auto_route::__private::inventory::submit! {
                    ::auto_route::OpenApiRouteDescriptor::new(
                        #method,
                        #path,
                        #operation_id,
                        #tag,
                        #params,
                        #request_schema,
                        #response_schema,
                    )
                }
            }
        })
        .collect::<Vec<_>>();
    items.push(generated);
    items.push(submission);
    for submission in openapi_submissions {
        items.push(syn::parse2(submission)?);
    }
    Ok(quote!(#item_mod))
}

fn expand_controller(
    base_path: LitStr,
    mut item_impl: ItemImpl,
) -> syn::Result<proc_macro2::TokenStream> {
    if item_impl.trait_.is_some() {
        return Err(syn::Error::new_spanned(
            &item_impl,
            "#[controller] requires an inherent impl block",
        ));
    }
    if !item_impl.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.generics,
            "generic controller impl blocks are not supported",
        ));
    }

    let self_ty = item_impl.self_ty.as_ref().clone();
    let has_singleton = item_impl.attrs.iter().any(|attribute| {
        attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "singleton")
    });
    let type_ident = type_ident(&self_ty)?;
    let factory_ident = format_ident!("__auto_route_factory_{}", type_ident);
    let mut routes = Vec::new();

    for impl_item in &mut item_impl.items {
        let ImplItem::Fn(function) = impl_item else {
            continue;
        };

        let mut route_attributes = Vec::new();
        let mut retained_attributes = Vec::new();
        for attribute in std::mem::take(&mut function.attrs) {
            if let Some(method) = route_method(&attribute) {
                route_attributes.push((attribute, method));
            } else {
                retained_attributes.push(attribute);
            }
        }
        function.attrs = retained_attributes;

        if route_attributes.len() > 1 {
            return Err(syn::Error::new(
                function.sig.span(),
                "a controller method can have only one route attribute",
            ));
        }
        let Some((attribute, method)) = route_attributes.pop() else {
            continue;
        };

        validate_route_signature(&function.sig, "controller route methods")?;

        let mut inputs = function.sig.inputs.iter();
        match inputs.next() {
            Some(FnArg::Receiver(receiver)) if receiver.reference.is_some() => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    &function.sig,
                    "controller route methods must take &self as their first argument",
                ));
            }
        }

        let argument_types = inputs
            .map(|argument| match argument {
                FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
                FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                    receiver,
                    "unexpected receiver argument",
                )),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let route_path = marker_path(&attribute)?;
        let full_path = join_paths(&base_path.value(), &route_path);

        routes.push(Route {
            method,
            handler: function.sig.ident.clone(),
            path: LitStr::new(&full_path, attribute.span()),
            request_body: infer_request_body(&argument_types),
            response_body: infer_response_body(&function.sig.output),
            params: infer_params(&full_path, &argument_types),
            argument_types,
        });
    }

    if routes.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.self_ty,
            "controller contains no route methods",
        ));
    }

    let registrations = routes.iter().map(|route| {
        let method = &route.method;
        let handler = &route.handler;
        let path = &route.path;
        let arguments = route
            .argument_types
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let name = format_ident!("__auto_route_arg_{index}");
                quote!(#name: #ty)
            })
            .collect::<Vec<_>>();
        let argument_names = (0..route.argument_types.len())
            .map(|index| format_ident!("__auto_route_arg_{index}"))
            .collect::<Vec<_>>();

        quote! {
            router = router.route(
                #path,
                ::auto_route::__private::axum::routing::#method({
                    let controller = ::std::sync::Arc::clone(&controller);
                    move |#(#arguments),*| {
                        let controller = ::std::sync::Arc::clone(&controller);
                        async move {
                            let response = controller.#handler(#(#argument_names),*).await;
                            ::auto_route::__private::axum::response::IntoResponse::into_response(response)
                        }
                    }
                }),
            );
        }
    });
    let openapi_submissions = routes.iter().map(|route| {
        let method = route.method.to_string().to_ascii_uppercase();
        let path = &route.path;
        let handler = &route.handler;
        let operation_id = format!("{type_ident}::{handler}");
        let tag = openapi_tag(&path.value());
        let params = param_descriptor_tokens(&route.params);
        let request_schema = request_schema_descriptor_tokens(route.request_body.as_ref());
        let response_schema = response_schema_descriptor_tokens(route.response_body.as_ref());
        quote! {
            ::auto_route::__private::inventory::submit! {
                ::auto_route::OpenApiRouteDescriptor::new(
                    #method,
                    #path,
                    #operation_id,
                    #tag,
                    #params,
                    #request_schema,
                    #response_schema,
                )
            }
        }
    });

    let managed_impl = if has_singleton {
        quote!(#item_impl)
    } else {
        quote! {
            #[auto_route::__private::auto_di::singleton]
            #item_impl
        }
    };

    Ok(quote! {
        #managed_impl

        #[doc(hidden)]
        #[allow(non_snake_case)]
        fn #factory_ident<'a>(
            container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let controller = container.resolve::<#self_ty>().await?;
                let mut router = ::auto_route::__private::axum::Router::new();
                #(#registrations)*
                Ok(router)
            })
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(#factory_ident)
        }

        #(#openapi_submissions)*
    })
}

fn validate_route_function(function: &ItemFn, label: &str) -> syn::Result<()> {
    validate_route_signature(&function.sig, label)
}

fn route_argument_types<'a>(inputs: impl Iterator<Item = &'a FnArg>) -> syn::Result<Vec<Type>> {
    inputs
        .map(|argument| match argument {
            FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "module route functions cannot take self",
            )),
        })
        .collect()
}

fn infer_request_body(arguments: &[Type]) -> Option<RequestBody> {
    arguments.iter().find_map(|argument| {
        wrapper_inner_type(argument, &["Json", "ValidatedJson"])
            .map(|ty| RequestBody {
                ty,
                content: RequestContent::Json,
            })
            .or_else(|| {
                wrapper_inner_type(argument, &["Form"]).map(|ty| RequestBody {
                    ty,
                    content: RequestContent::Form,
                })
            })
    })
}

fn infer_response_body(output: &ReturnType) -> Option<ResponseBody> {
    match output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => response_body_type(ty),
    }
}

fn response_body_type(ty: &Type) -> Option<ResponseBody> {
    if is_sse_response_type(ty) {
        return Some(ResponseBody::Sse);
    }

    if let Some(inner) = wrapper_inner_type(ty, &["Json"]) {
        return Some(ResponseBody::Json(inner));
    }

    match ty {
        Type::Paren(paren) => response_body_type(&paren.elem),
        Type::Reference(reference) => response_body_type(&reference.elem),
        Type::Tuple(tuple) => tuple.elems.iter().find_map(response_body_type),
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            if segment.ident != "Result" && segment.ident != "Option" {
                return None;
            }
            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            arguments.args.iter().find_map(|argument| match argument {
                GenericArgument::Type(ty) => response_body_type(ty),
                _ => None,
            })
        }
        _ => None,
    }
}

fn is_sse_response_type(ty: &Type) -> bool {
    match ty {
        Type::Paren(paren) => is_sse_response_type(&paren.elem),
        Type::Reference(reference) => is_sse_response_type(&reference.elem),
        Type::Path(type_path) => type_path.path.segments.last().is_some_and(|segment| {
            let ident = segment.ident.to_string();
            ident == "Sse"
                || ident.ends_with("Sse")
                || ident.ends_with("EventStream")
                || ident.ends_with("EventStreamResponse")
                || ident.ends_with("EventsStream")
                || ident.ends_with("EventsStreamResponse")
        }),
        _ => false,
    }
}

fn infer_params(path: &str, arguments: &[Type]) -> Vec<OpenApiParam> {
    let mut params = infer_path_params(path, arguments);
    params.extend(arguments.iter().filter_map(|argument| {
        wrapper_inner_type(argument, &["Query"]).map(|ty| OpenApiParam {
            name: "query".to_owned(),
            ty,
            source: ParamSource::Query,
        })
    }));
    params
}

fn infer_path_params(path: &str, arguments: &[Type]) -> Vec<OpenApiParam> {
    let names = path_parameter_names(path);
    let path_types = arguments
        .iter()
        .find_map(|argument| wrapper_inner_type(argument, &["Path"]))
        .map(|ty| path_inner_types(&ty))
        .unwrap_or_default();

    names
        .into_iter()
        .enumerate()
        .map(|(index, name)| OpenApiParam {
            name,
            ty: path_types.get(index).cloned().unwrap_or_else(string_type),
            source: ParamSource::Path,
        })
        .collect()
}

fn path_inner_types(ty: &Type) -> Vec<Type> {
    match ty {
        Type::Tuple(tuple) => tuple.elems.iter().cloned().collect(),
        Type::Paren(paren) => path_inner_types(&paren.elem),
        Type::Reference(reference) => path_inner_types(&reference.elem),
        _ => vec![ty.clone()],
    }
}

fn path_parameter_names(path: &str) -> Vec<String> {
    path.split('/')
        .filter_map(|segment| {
            Some(
                segment
                    .strip_prefix('{')?
                    .strip_suffix('}')?
                    .trim_start_matches('*')
                    .to_owned(),
            )
        })
        .collect()
}

fn wrapper_inner_type(ty: &Type, wrappers: &[&str]) -> Option<Type> {
    match ty {
        Type::Paren(paren) => wrapper_inner_type(&paren.elem, wrappers),
        Type::Reference(reference) => wrapper_inner_type(&reference.elem, wrappers),
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };

            if segment.ident == "Option" {
                return arguments.args.iter().find_map(|argument| match argument {
                    GenericArgument::Type(ty) => wrapper_inner_type(ty, wrappers),
                    _ => None,
                });
            }

            if !wrappers.iter().any(|wrapper| segment.ident == wrapper) {
                return None;
            }

            arguments.args.iter().find_map(|argument| match argument {
                GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            })
        }
        _ => None,
    }
}

fn string_type() -> Type {
    syn::parse_quote!(::std::string::String)
}

fn request_schema_descriptor_tokens(body: Option<&RequestBody>) -> proc_macro2::TokenStream {
    match body {
        Some(RequestBody {
            ty,
            content: RequestContent::Json,
        }) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::json::<#ty>())
        },
        Some(RequestBody {
            ty,
            content: RequestContent::Form,
        }) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::form::<#ty>())
        },
        None => quote!(::std::option::Option::None),
    }
}

fn response_schema_descriptor_tokens(body: Option<&ResponseBody>) -> proc_macro2::TokenStream {
    match body {
        Some(ResponseBody::Json(ty)) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::json::<#ty>())
        },
        Some(ResponseBody::Sse) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::sse())
        },
        None => quote!(::std::option::Option::None),
    }
}

fn param_descriptor_tokens(params: &[OpenApiParam]) -> proc_macro2::TokenStream {
    let params = params.iter().map(|param| {
        let name = LitStr::new(&param.name, proc_macro2::Span::call_site());
        let ty = &param.ty;
        match param.source {
            ParamSource::Path => quote!(::auto_route::OpenApiParamDescriptor::path::<#ty>(#name)),
            ParamSource::Query => quote!(::auto_route::OpenApiParamDescriptor::query::<#ty>(#name)),
        }
    });

    quote!(&[#(#params),*])
}

fn validate_route_signature(signature: &syn::Signature, label: &str) -> syn::Result<()> {
    if signature.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            signature,
            format!("{label} must be async"),
        ));
    }
    if !signature.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &signature.generics,
            format!("generic {label} are not supported"),
        ));
    }
    Ok(())
}

fn marker_path(attribute: &Attribute) -> syn::Result<String> {
    attribute
        .parse_args::<LitStr>()
        .map(|path| path.value())
        .or_else(|error| {
            if matches!(&attribute.meta, syn::Meta::Path(_)) {
                Ok(String::new())
            } else {
                Err(error)
            }
        })
}

fn route_method(attribute: &Attribute) -> Option<syn::Ident> {
    let ident = attribute.path().get_ident()?;
    METHODS
        .contains(&ident.to_string().as_str())
        .then(|| ident.clone())
}

fn type_ident(ty: &Type) -> syn::Result<&syn::Ident> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "controller type must be a named type",
        ));
    };
    path.path
        .segments
        .last()
        .map(|segment| &segment.ident)
        .ok_or_else(|| syn::Error::new_spanned(ty, "controller type must be a named type"))
}

fn join_paths(base: &str, route: &str) -> String {
    let base = base.trim_matches('/');
    let route = route.trim_matches('/');
    let joined = match (base.is_empty(), route.is_empty()) {
        (true, true) => "/".to_owned(),
        (false, true) => format!("/{base}"),
        (true, false) => format!("/{route}"),
        (false, false) => format!("/{base}/{route}"),
    };

    joined
        .split('/')
        .map(|segment| {
            if let Some(parameter) = segment.strip_prefix(':') {
                format!("{{{parameter}}}")
            } else if let Some(parameter) = segment.strip_prefix('*') {
                format!("{{*{parameter}}}")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn openapi_tag(path: &str) -> String {
    path.trim_start_matches('/')
        .split('/')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("default")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::join_paths;

    #[test]
    fn joins_and_converts_spring_style_parameters() {
        assert_eq!(join_paths("/users/", "/:id"), "/users/{id}");
        assert_eq!(join_paths("", ""), "/");
        assert_eq!(join_paths("/files", "/*path"), "/files/{*path}");
    }
}
