use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn ts_interop(_: TokenStream, item: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    quote! {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[cfg_attr(
        feature = "wasm",
        derive(tsify::Tsify),
        tsify(into_wasm_abi, from_wasm_abi)
    )]
    #item
    }
    .into()
}
