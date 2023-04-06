use solana_program::{
    program_error::ProgramError, program_option::COption, program_pack::Pack, pubkey::Pubkey,
};

pub trait MintInterface: MintDeserialize {
    fn mint_authority(&self) -> COption<Pubkey>;
    fn supply(&self) -> u64;
    fn decimals(&self) -> u8;
    fn is_initialized(&self) -> bool;
    fn freeze_authority(&self) -> COption<Pubkey>;
}

pub trait MintDeserialize: Sized + From<spl_token::state::Mint> {
    // TODO: Implement byte-wise deserialization via discriminator
    //
    fn unpack_mint(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(spl_token::state::Mint::unpack(input)?.into())
    }
}
