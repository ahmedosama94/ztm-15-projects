use proc_macro::TokenStream;

#[proc_macro_derive(SimpleEnumToString)]
fn simple_enum_to_string_macro_derive(input: TokenStream) -> TokenStream {
    let ast = &syn::parse(input).expect("Unable to parse derive macro input");

    impl_simple_enum_to_string_macro(ast)
}

fn impl_simple_enum_to_string_macro(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    println!("{:#?}", ast.data);

    todo!()
}
