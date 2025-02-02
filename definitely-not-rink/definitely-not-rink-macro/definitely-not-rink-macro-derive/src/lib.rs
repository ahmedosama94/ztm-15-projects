use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(FromWithPrefix)]
pub fn from_with_prefix_macro_derive(input: TokenStream) -> TokenStream {
    let ast = &syn::parse(input).unwrap();

    impl_from_with_prefix_macro(ast)
}

fn impl_from_with_prefix_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl FromWithPrefix for #name {
            fn test(&self) {
                println!("Testing");
            }
        }
    };

    gen.into()
}
