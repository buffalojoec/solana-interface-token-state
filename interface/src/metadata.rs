use mpl_token_metadata::state::TokenMetadataAccount;
use solana_program::{program_error::ProgramError, program_option::COption, pubkey::Pubkey};

pub trait MetadataInterface {
    fn title(&self) -> &String;
    fn symbol(&self) -> &String;
    fn uri(&self) -> &String;
    fn update_authority(&self) -> Pubkey;
}

pub trait MetadataInterfacePack<'a>
where
    &'a Self: Into<mpl_token_metadata::state::Metadata> + 'a,
    Self: Sized,
{
    const DISCRIMINATOR: &'static str = "metadata_state";

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

    fn unpack_metadata(input: &[u8]) -> Result<mpl_token_metadata::state::Metadata, ProgramError> {
        let discrim = &Self::discriminator_slice();
        match input
            .windows(discrim.len())
            .find(|&window| window == discrim)
            .map(|match_start| &input[match_start.len()..])
        {
            Some(metadata_buffer) => Ok(mpl_token_metadata::state::Metadata::safe_deserialize(
                metadata_buffer,
            )?),
            None => Err(ProgramError::BorshIoError(
                "Error: Failed to unpack Metadata data from this account.".to_string(),
            )),
        }
    }

    fn pack_metadata(&'a self, buf: &mut [u8]) -> Result<(), ProgramError> {
        let mut metadata_vec = vec![];
        for b in Self::discriminator_slice().iter() {
            metadata_vec.push(*b);
        }
        for _ in 0..mpl_token_metadata::state::Metadata::size() {
            metadata_vec.push(0u8);
        }
        let metadata_buf = metadata_vec.as_mut_slice();
        mpl_token_metadata::state::Metadata::save(
            &<&'a Self as Into<mpl_token_metadata::state::Metadata>>::into(self),
            &mut metadata_buf[8..],
        )?;
        let len = buf.len();
        buf[len..].copy_from_slice(&metadata_buf);
        Ok(())
    }
}
