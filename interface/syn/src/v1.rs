use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, ItemStruct};

use crate::{impl_trait::impl_borsh, interface::ImplementedInterface, parser::parse_attributes};

pub struct StateInterfaceItemStructV1 {
    pub item_struct: ItemStruct,
    pub implemented_interfaces: Vec<ImplementedInterface>,
}

impl StateInterfaceItemStructV1 {
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

impl From<ItemStruct> for StateInterfaceItemStructV1 {
    fn from(value: ItemStruct) -> Self {
        Self {
            item_struct: value.clone(),
            implemented_interfaces: parse_attributes(&value),
        }
    }
}

impl Parse for StateInterfaceItemStructV1 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ItemStruct::parse(input)?.into())
    }
}

impl ToTokens for StateInterfaceItemStructV1 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

// This is the only piece that changes between V1 & V2
//  V2 adds the additional fields automatically
//  Everything else is the same
//
impl From<&StateInterfaceItemStructV1> for TokenStream {
    fn from(value: &StateInterfaceItemStructV1) -> Self {
        value.dedupe();
        let ident = &value.item_struct.ident;
        let fields = &value.item_struct.fields;
        let _impl_borsh = impl_borsh(&value.item_struct);
        let impl_traits = value.implemented_interfaces.iter().map(|i| {
            i.check_fields(fields);
            i.get_impl_traits(ident)
        });
        quote! {
            // #impl_borsh
            #(#impl_traits)*
        }
    }
}
