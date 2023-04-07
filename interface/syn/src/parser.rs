use syn::{Attribute, ItemStruct};

use crate::interface::ImplementedInterface;

pub fn parse_attributes(item_struct: &ItemStruct) -> Vec<ImplementedInterface> {
    let mut implemented_interfaces = Vec::new();

    for attr in item_struct.attrs.iter() {
        if let Ok(syn::Meta::List(ref meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("state_interfaces") {
                for nested_meta in meta_list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = nested_meta {
                        let variant_string = path.get_ident().unwrap().to_string();
                        match ImplementedInterface::try_from(&variant_string) {
                            Ok(impl_interface) => implemented_interfaces.push(impl_interface),
                            Err(_) => panic!("Error: Unknown interface: `{}`", variant_string),
                        };
                    } else {
                        panic!("Error: Invalid format for `state_interfaces` attribute.");
                    }
                }
            }
        }
    }
    implemented_interfaces
}

pub fn filter_out_interface_attributes(attrs: &Vec<Attribute>) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|a| {
            if let Ok(syn::Meta::List(ref meta_list)) = a.parse_meta() {
                if meta_list.path.is_ident("state_interfaces") {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect()
}
