use syn::parse::{Parse, Parser};
use syn::{Expr, LitStr, Token};

struct MdArgs {
    ui: Expr,
    md: LitStr,
}

impl Parse for MdArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ui: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let md: LitStr = input.parse()?;
        Ok(MdArgs { ui, md })
    }
}

#[proc_macro]
pub fn md(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MdArgs { ui, md } = syn::parse_macro_input!(input);
    todo!()
}
