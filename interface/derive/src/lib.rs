use proc_macro::TokenStream;
use quote::ToTokens;
use state_interface_syn::{v1::StateInterfaceItemStructV1, v2::StateInterfaceItemStructV2};
use syn::parse_macro_input;

extern crate proc_macro;

#[proc_macro_derive(StateInterfaceV1, attributes(state_interfaces))]
pub fn state_interface_v1(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as StateInterfaceItemStructV1)
        .to_token_stream()
        .into()
}

#[proc_macro_attribute]
pub fn state_interface_v2(_: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as StateInterfaceItemStructV2)
        .to_token_stream()
        .into()
}
