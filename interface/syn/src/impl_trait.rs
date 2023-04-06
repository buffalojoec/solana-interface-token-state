use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemStruct};

pub fn impl_borsh(item_struct: &ItemStruct) -> TokenStream {
    let ident = &item_struct.ident;
    let fields = &item_struct.fields;
    let borsh_ser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote! { #field_ty: borsh::ser::BorshSerialize }
    });
    let borsh_ser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote! { borsh::BorshSerialize::serialize(&self.#field_name, writer)? }
    });
    let borsh_deser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote! { #field_ty: borsh::de::BorshDeserialize }
    });
    let borsh_deser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote! { #field_name: borsh::BorshDeserialize::deserialize(buf)? }
    });
    quote! {
        impl borsh::ser::BorshSerialize for #ident
        where
            #(#borsh_ser_where,)*
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                #(#borsh_ser_impl;)*
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for #ident
        where
            #(#borsh_deser_where,)*
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    #(#borsh_deser_impl,)*
                })
            }
        }
    }
}

pub fn implement_metadata(ident: &Ident) -> TokenStream {
    quote! {
        impl state_interface::metadata::MetadataInterface for #ident {
            fn title(&self) -> String {
                self.title
            }

            fn symbol(&self) -> String {
                self.symbol
            }

            fn uri(&self) -> String {
                self.uri
            }

            fn update_authority(
                &self,
            ) -> solana_program::program_option::COption<solana_program::pubkey::Pubkey> {
                self.update_authority
            }
        }
    }
}

pub fn implement_mint(ident: &Ident) -> TokenStream {
    quote! {
        impl state_interface::mint::MintInterface for #ident {
            fn mint_authority(&self) -> solana_program::program_option::COption<solana_program::pubkey::Pubkey> {
                self.mint_authority
            }

            fn supply(&self) -> u64 {
                self.supply
            }

            fn decimals(&self) -> u8 {
                self.decimals
            }

            fn is_initialized(&self) -> bool {
                self.is_initialized
            }

            fn freeze_authority(&self) -> solana_program::program_option::COption<solana_program::pubkey::Pubkey> {
                self.freeze_authority
            }
        }
    }
}
