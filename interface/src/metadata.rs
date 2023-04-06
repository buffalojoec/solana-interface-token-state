use solana_program::{program_error::ProgramError, program_option::COption, pubkey::Pubkey};

pub trait MetadataInterface: MintDeserialize {
    fn title(&self) -> String;
    fn symbol(&self) -> String;
    fn uri(&self) -> String;
    fn update_authority(&self) -> COption<Pubkey>;
}

pub trait MintDeserialize: Sized + From<mpl_token_metadata::state::Metadata> {
    // TODO: Implement byte-wise deserialization via discriminator
    //
    fn unpack_metadata(input: &[u8]) -> Result<Self, ProgramError> {
        use mpl_token_metadata::state::TokenMetadataAccount;
        Ok(mpl_token_metadata::state::Metadata::safe_deserialize(input)?.into())
    }
}
