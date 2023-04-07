pub use mpl_token_metadata::state::{
    Collection, CollectionDetails, Data, Key, Metadata, ProgrammableConfig, TokenStandard, Uses,
};
use solana_program::program_error::ProgramError;
pub use spl_token::state::Mint;
pub use state_interface_derive::*;

pub mod metadata;
pub mod mint;

pub use metadata::{MetadataInterface, MetadataInterfacePack};
pub use mint::{MintInterface, MintInterfacePack};

pub enum Interface {
    Metadata,
    Mint,
    None,
}

pub trait InterfacePack<'a>: Sized {
    const IMPLEMENTED_INTERFACES: [Interface; 2];

    fn unpack(buf: &[u8]) -> Result<Self, ProgramError> {
        for interface in Self::IMPLEMENTED_INTERFACES {
            // This trait might not work.
            // We need to, for each interface implemented, unpack that struct,
            //      then move to the rest of the bytes.
            // Once we've unpacked all the interface types, unpack the rest into
            //      any additional fields.
            // Then return the entire data structure.
            ()
        }
        todo!()
    }

    fn pack<W: std::io::Write>(&'a self, writer: &mut W) -> Result<(), ProgramError> {
        for interface in Self::IMPLEMENTED_INTERFACES {
            // This one should work.
            // We need to, for each interface implemented, pack the fields from the
            //      interface into bytes.
            //      (We can do this with &self.into())
            // Then we need to pack the rest of the struct into bytes.
            // Then append the whole slice with all slices concatenated.
            ()
        }
        Ok(())
    }
}
