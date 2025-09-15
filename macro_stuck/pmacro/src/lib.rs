use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn pmacro(_: TokenStream, item: TokenStream) -> TokenStream {
    format!("#[derive(pmacro::Attr)] {item}").parse().unwrap()
}

#[proc_macro_derive(Attr, attributes(foo))]
pub fn derive_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
