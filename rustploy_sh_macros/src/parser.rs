use syn::parse::Parse;
use syn::Stmt;

pub struct ShInput {
    pub stmts: Vec<Stmt>,
}

impl Parse for ShInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut stmts = Vec::new();
        while !input.is_empty() {
            stmts.push(input.parse()?);
        }
        Ok(ShInput { stmts })
    }
}
