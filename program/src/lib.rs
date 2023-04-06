use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_option::COption,
    pubkey::Pubkey,
};
use state_interface::{
    metadata::MetadataInterface, mint::MintInterface, state_interface_v2, Metadata, Mint,
    StateInterfaceV1,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum MyInstruction {
    UseV1,
    UseV2,
    UseV3,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(input)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    match instruction {
        MyInstruction::UseV1 => {
            msg!("Now Using V1");

            // Unpack the account data according to your struct
            let token_data = JoeTokenV1::try_from_slice(&account.data.borrow())?;

            // Unpack just the mint data
            let mint_data = MintInterface::unpack_mint(&account.data.borrow())?;

            token_data.log(
                Some(mint_data),
                None,
                token_data.slot_created,
                token_data.largest_mint,
                token_data.organization,
            )
        }
        MyInstruction::UseV2 => {
            msg!("Now Using V2");

            // Unpack the account data according to your struct
            let token_data = JoeTokenV2::try_from_slice(&account.data.borrow())?;

            // Unpack just the mint data
            let mint_data = MintInterface::unpack_mint(&account.data.borrow())?;

            token_data.log(
                Some(mint_data),
                None,
                token_data.slot_created,
                token_data.largest_mint,
                token_data.organization,
            )
        }
        MyInstruction::UseV3 => {
            msg!("Now Using V3");

            // Unpack the account data according to your struct
            let token_data = JoeTokenV1::try_from_slice(&account.data.borrow())?;

            // Unpack just the mint data
            let mint_data = MintInterface::unpack_mint(&account.data.borrow())?;

            // Unpack just the metadata data
            let metadata_data = MetadataInterface::unpack_metadata(&account.data.borrow())?;

            token_data.log(
                Some(mint_data),
                Some(metadata_data),
                token_data.slot_created,
                token_data.largest_mint,
                token_data.organization,
            )
        }
    };
}

#[derive(StateInterfaceV1)]
#[state_interfaces(Mint)]
pub struct JoeTokenV1 {
    pub mint_authority: COption<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: COption<Pubkey>,
    //
    pub slot_created: u64,
    pub largest_mint: u64,
    pub organization: String,
}

#[state_interface_v2]
#[state_interfaces(Mint)]
pub struct JoeTokenV2 {
    // Mint fields are automatically added
    //
    pub slot_created: u64,
    pub largest_mint: u64,
    pub organization: String,
}

#[state_interface_v2]
#[state_interfaces(Mint, Metadata)]
pub struct JoeTokenV3 {
    // Mint & Metadata fields are automatically added
    //
    pub slot_created: u64,
    pub largest_mint: u64,
    pub organization: String,
}

//

trait Logger: Sized {
    fn log_self(self);
    fn log(
        self,
        mint: Option<Mint>,
        metadata: Option<Metadata>,
        slot_created: u64,
        largest_mint: u64,
        organization: String,
    ) -> ProgramResult {
        self.log_self();
        mint.map_or((), |mint| print_mint_details(mint));
        metadata.map_or((), |metadata| print_metadata_details(metadata));
        print_additional_details(slot_created, largest_mint, organization);
        Ok(())
    }
}

impl Logger for JoeTokenV1 {
    fn log_self(self) {
        print_token_details_as_mint(self);
    }
}

impl Logger for JoeTokenV2 {
    fn log_self(self) {
        print_token_details_as_mint(self);
    }
}

impl Logger for JoeTokenV3 {
    fn log_self(self) {
        print_token_details_as_mint(self);
        print_token_details_as_metadata(self);
    }
}

fn print_mint_details(mint: Mint) {
    msg!("Mint Authority:       {:#?}", mint.mint_authority);
    msg!("Supply:               {}", mint.supply);
    msg!("Decimals:             {}", mint.decimals);
    msg!("Is Initialized:       {}", mint.is_initialized);
    msg!("Freeze Authority:     {:#?}", mint.freeze_authority);
}

fn print_metadata_details(metadata: Metadata) {
    msg!("Title:                {}", metadata.data.name);
    msg!("Symbol:               {}", metadata.data.symbol);
    msg!("URI:                  {}", metadata.data.uri);
    msg!("Update Authority:     {:#?}", metadata.update_authority);
}

fn print_token_details_as_mint(token: impl MintInterface) {
    msg!("Mint Authority:       {:#?}", token.mint_authority());
    msg!("Supply:               {}", token.supply());
    msg!("Decimals:             {}", token.decimals());
    msg!("Is Initialized:       {}", token.is_initialized());
    msg!("Freeze Authority:     {:#?}", token.freeze_authority());
}

fn print_token_details_as_metadata(token: impl MetadataInterface) {
    msg!("Title:                {}", token.title());
    msg!("Symbol:               {}", token.symbol());
    msg!("URI:                  {}", token.uri());
    msg!("Update Authority:     {:#?}", token.update_authority());
}

fn print_additional_details(slot_created: u64, largest_mint: u64, organization: String) {
    msg!("-- Additional details:");
    msg!("Slot Created:         {}", slot_created);
    msg!("Largest Mint:         {}", largest_mint);
    msg!("Organization:         {}", organization);
}
