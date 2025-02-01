use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(FromFahrenheit)]
pub fn from_fahernheit_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_fahrenheit_macro(&ast)
}

fn impl_from_fahrenheit_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<TemperatureFahrenheit> for #name {
            fn from(value: TemperatureFahrenheit) -> Self {
                Self(Self::from_default(value.to_default()).value())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(FromCelsius)]
pub fn from_celsius_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_celsius_macro(&ast)
}

fn impl_from_celsius_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<TemperatureCelsius> for #name {
            fn from(value: TemperatureCelsius) -> Self {
                Self(Self::from_default(value.to_default()).value())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(FromKelvin)]
pub fn from_kelvin_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_kelvin_macro(&ast)
}

fn impl_from_kelvin_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl From<TemperatureKelvin> for #name {
            fn from(value: TemperatureKelvin) -> Self {
                Self(Self::from_default(value.to_default()).value())
            }
        }
    };

    gen.into()
}