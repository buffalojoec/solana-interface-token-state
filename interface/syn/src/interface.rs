use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Field, Fields, Ident, Type};

use crate::impl_trait::{implement_metadata, implement_mint};

#[derive(PartialEq, Eq, Hash)]
pub enum ImplementedInterface {
    Metadata,
    Mint,
}

impl TryFrom<&String> for ImplementedInterface {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.eq(&ImplementedInterface::Metadata.to_string()) {
            Ok(ImplementedInterface::Metadata)
        } else if value.eq(&ImplementedInterface::Mint.to_string()) {
            Ok(ImplementedInterface::Mint)
        } else {
            Err("String is not a match")
        }
    }
}

impl ToString for ImplementedInterface {
    fn to_string(&self) -> String {
        match self {
            ImplementedInterface::Metadata => "Metadata".to_string(),
            ImplementedInterface::Mint => "Mint".to_string(),
        }
    }
}

impl ImplementedInterface {
    pub fn check_fields(&self, fields: &Fields) {
        match self {
            ImplementedInterface::Metadata => {
                match crate::check_fields::check_fields(fields, self.get_fields()) {
                    Ok(()) => (),
                    Err(e) => panic!("{}", e),
                }
            }
            ImplementedInterface::Mint => {
                match crate::check_fields::check_fields(fields, self.get_fields()) {
                    Ok(()) => (),
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }

    pub fn get_fields(&self) -> Vec<Field> {
        match self {
            ImplementedInterface::Metadata => vec![
                new_field("title", parse_quote! { String }),
                new_field("symbol", parse_quote! { String }),
                new_field("uri", parse_quote! { String }),
                new_field("update_authority", parse_quote! { COption<Pubkey> }),
            ],
            ImplementedInterface::Mint => vec![
                new_field("mint_authority", parse_quote! { COption<Pubkey> }),
                new_field("supply", parse_quote! { u64 }),
                new_field("decimals", parse_quote! { u8 }),
                new_field("is_initialized", parse_quote! { bool }),
                new_field("freeze_authority", parse_quote! { COption<Pubkey> }),
            ],
        }
    }

    pub fn get_fields_tokens(&self) -> TokenStream {
        match self {
            ImplementedInterface::Metadata => quote! {
                pub title: String,
                pub symbol: String,
                pub uri: String,
                pub update_authority: COption<Pubkey>,
            },
            ImplementedInterface::Mint => quote! {
                pub mint_authority: COption<Pubkey>,
                pub supply: u64,
                pub decimals: u8,
                pub is_initialized: bool,
                pub freeze_authority: COption<Pubkey>,
            },
        }
    }

    pub fn get_impl_traits(&self, ident: &Ident) -> TokenStream {
        match self {
            ImplementedInterface::Metadata => implement_metadata(ident),
            ImplementedInterface::Mint => implement_mint(ident),
        }
    }
}

fn new_field(name: &str, ty: Type) -> Field {
    Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new(name, Span::call_site())),
        colon_token: Some(Default::default()),
        ty,
    }
}
