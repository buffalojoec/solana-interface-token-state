use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, ItemStruct};

use crate::{
    impl_traits::impl_interface_pack,
    interface::ImplementedInterface,
    parser::{filter_out_interface_attributes, parse_attributes},
};

pub struct StateInterfaceItemStructV2 {
    pub item_struct: ItemStruct,
    pub implemented_interfaces: Vec<ImplementedInterface>,
}

impl StateInterfaceItemStructV2 {
    pub fn dedupe(&self) {
        let mut seen = std::collections::HashSet::new();
        for interface in &self.implemented_interfaces {
            if !seen.insert(interface) {
                panic!(
                    "Error: Can't declare an interface twice: `{}`",
                    interface.to_string()
                )
            }
        }
    }
}

impl From<ItemStruct> for StateInterfaceItemStructV2 {
    fn from(value: ItemStruct) -> Self {
        Self {
            item_struct: value.clone(),
            implemented_interfaces: parse_attributes(&value),
        }
    }
}

impl Parse for StateInterfaceItemStructV2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ItemStruct::parse(input)?.into())
    }
}

impl ToTokens for StateInterfaceItemStructV2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

// This is the only piece that changes between V1 & V2
//  V2 adds the additional fields automatically
//  Everything else is the same
//
impl From<&StateInterfaceItemStructV2> for TokenStream {
    fn from(value: &StateInterfaceItemStructV2) -> Self {
        value.dedupe();
        let ident = &value.item_struct.ident;
        let attrs = filter_out_interface_attributes(&value.item_struct.attrs);
        let fields = value.item_struct.fields.iter();
        let impl_interface_pack =
            impl_interface_pack(&value.item_struct, &value.implemented_interfaces);
        let (added_fields, impl_traits): (Vec<TokenStream>, Vec<TokenStream>) = value
            .implemented_interfaces
            .iter()
            .map(|i| (i.get_fields_tokens(), i.get_impl_traits(ident)))
            .unzip();
        quote! {
            #(#attrs)*
            pub struct #ident {
                #(#added_fields)*
                #(#fields,)*
            }
            #impl_interface_pack
            #(#impl_traits)*
        }
    }
}
