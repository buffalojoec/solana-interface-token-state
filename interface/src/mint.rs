use solana_program::{
    program_error::ProgramError, program_option::COption, program_pack::Pack, pubkey::Pubkey,
};

pub trait MintInterface {
    fn mint_authority(&self) -> COption<Pubkey>;
    fn supply(&self) -> u64;
    fn decimals(&self) -> u8;
    fn is_initialized(&self) -> bool;
    fn freeze_authority(&self) -> COption<Pubkey>;
}

pub trait MintInterfacePack<'a>
where
    &'a Self: Into<spl_token::state::Mint> + 'a,
    Self: Sized,
{
    const DISCRIMINATOR: &'static str = "mint_state";

    fn discriminator_slice() -> [u8; 8] {
        let bytes = Self::DISCRIMINATOR.as_bytes();
        let mut discrim = [0u8; 8];
        for i in 0..8 {
            if i < bytes.len() {
                discrim[i] = bytes[i];
            }
        }
        discrim
    }

    fn unpack_mint(input: &[u8]) -> Result<Option<spl_token::state::Mint>, ProgramError> {
        let discrim = &Self::discriminator_slice();
        match input
            .windows(discrim.len())
            .find(|&window| window == discrim)
            .map(|match_start| &input[match_start.len()..])
        {
            Some(mint_buffer) => Ok(Some(spl_token::state::Mint::unpack(mint_buffer)?)),
            None => Ok(None),
        }
    }

    fn pack_mint(&'a self, buf: &mut [u8]) -> Result<(), ProgramError> {
        let mut mint_buf = [0u8; 8 + spl_token::state::Mint::LEN];
        for (i, b) in Self::discriminator_slice().iter().enumerate() {
            mint_buf[i] = *b
        }
        spl_token::state::Mint::pack(
            <&'a Self as Into<spl_token::state::Mint>>::into(self),
            &mut mint_buf[8..],
        )?;
        let len = buf.len();
        buf[len..].copy_from_slice(&mint_buf);
        Ok(())
    }
}
