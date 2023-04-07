use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemStruct};

use crate::interface::ImplementedInterface;

pub fn impl_interface_pack(
    item_struct: &ItemStruct,
    implemented_interfaces: &Vec<ImplementedInterface>,
) -> TokenStream {
    let ident = &item_struct.ident;
    let impl_interfaces_len = implemented_interfaces.len();
    let impl_src_interfaces = implemented_interfaces
        .iter()
        .map(|i| i.to_src_interface_tokens());
    let none_slots = 2 - impl_interfaces_len;
    let impl_src_interfaces_const_tokens = match none_slots > 0 {
        true => {
            let none_slot_tokens = {
                let mut none_vec = vec![];
                for _ in 0..none_slots {
                    none_vec.push(quote! { state_interface::Interface::None })
                }
                none_vec.into_iter()
            };
            quote! { [#(#impl_src_interfaces,)* #(#none_slot_tokens,)*]; }
        }
        false => quote! { [#(#impl_src_interfaces,)*]; },
    };
    quote! {
        impl<'a> state_interface::InterfacePack<'a> for #ident {
            const IMPLEMENTED_INTERFACES: [state_interface::Interface; 2] = #impl_src_interfaces_const_tokens
        }

        impl borsh::de::BorshDeserialize for #ident {
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                Self::unpack(buf).map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
            }
        }

        impl borsh::ser::BorshSerialize for #ident {
            fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                Self::pack(&self, writer).map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
            }
        }
    }
}

pub fn implement_metadata(ident: &Ident) -> TokenStream {
    quote! {
        impl<'a> From<&'a #ident> for state_interface::Metadata {
            fn from(value: &'a #ident) -> Self {
                Self {
                    key: value.key.clone(),
                    update_authority: value.update_authority.clone(),
                    mint: value.mint.clone(),
                    data: value.data.clone(),
                    primary_sale_happened: value.primary_sale_happened.clone(),
                    is_mutable: value.is_mutable.clone(),
                    edition_nonce: value.edition_nonce.clone(),
                    token_standard: value.token_standard.clone(),
                    collection: value.collection.clone(),
                    uses: value.uses.clone(),
                    collection_details: value.collection_details.clone(),
                    programmable_config: value.programmable_config.clone(),
                }
            }
        }

        impl state_interface::metadata::MetadataInterface for #ident {
            fn title(&self) -> &String {
                &self.data.name
            }

            fn symbol(&self) -> &String {
                &self.data.symbol
            }

            fn uri(&self) -> &String {
                &self.data.uri
            }

            fn update_authority(
                &self,
            ) -> solana_program::pubkey::Pubkey {
                self.update_authority
            }
        }

        impl<'a> state_interface::metadata::MetadataInterfacePack<'a> for #ident {}
    }
}

pub fn implement_mint(ident: &Ident) -> TokenStream {
    quote! {
        impl<'a> From<&'a #ident> for state_interface::Mint {
            fn from(value: &'a #ident) -> Self {
                Self {
                    mint_authority: value.mint_authority.clone(),
                    supply: value.supply.clone(),
                    decimals: value.decimals.clone(),
                    is_initialized: value.is_initialized.clone(),
                    freeze_authority: value.freeze_authority.clone(),
                }
            }
        }

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

        impl<'a> state_interface::mint::MintInterfacePack<'a> for #ident {}
    }
}
