use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_option::COption,
    pubkey::Pubkey,
    system_instruction,
};
use state_interface::{
    metadata::MetadataInterface, mint::MintInterface, state_interface_v2, Metadata, Mint,
    StateInterfaceV1,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum MyInstruction {
    CreateUsingV1(JoeTokenV1),
    CreateUsingV2(JoeTokenV2),
    CreateUsingV3(JoeTokenV3),
    ReadUsingV1,
    ReadUsingV2,
    ReadUsingV3,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(input)?;

    match instruction {
        MyInstruction::CreateUsingV1(data) => {
            let accounts_iter = &mut accounts.iter();
            let new_account = next_account_info(accounts_iter)?;
            let payer = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            create_pda(new_account, payer, system_program, program_id, data)
        }
        MyInstruction::CreateUsingV2(data) => {
            let accounts_iter = &mut accounts.iter();
            let new_account = next_account_info(accounts_iter)?;
            let payer = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            create_pda(new_account, payer, system_program, program_id, data)
        }
        MyInstruction::CreateUsingV3(data) => {
            let accounts_iter = &mut accounts.iter();
            let new_account = next_account_info(accounts_iter)?;
            let payer = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            create_pda(new_account, payer, system_program, program_id, data)
        }
        MyInstruction::ReadUsingV1 => {
            let accounts_iter = &mut accounts.iter();
            let account = next_account_info(accounts_iter)?;
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
        MyInstruction::ReadUsingV2 => {
            let accounts_iter = &mut accounts.iter();
            let account = next_account_info(accounts_iter)?;
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
        MyInstruction::ReadUsingV3 => {
            let accounts_iter = &mut accounts.iter();
            let account = next_account_info(accounts_iter)?;
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

trait Pda: BorshDeserialize {
    fn span(&self) -> usize {
        (self.try_to_vec()?).len()
    }
    fn size(&self) -> u64 {
        self.span().try_into().unwrap()
    }
    fn required_rent(&self) -> Result<u64, solana_program::program_error::ProgramError> {
        use solana_program::sysvar::Sysvar;
        Ok((solana_program::sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()))
    }
    fn seeds(&self) -> [&[u8]; 1];
    fn pda(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

impl Pda for JoeTokenV1 {}
impl Pda for JoeTokenV2 {}
impl Pda for JoeTokenV3 {}

//

pub fn create_pda<'a, T: Pda>(
    new_account: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    program_id: &Pubkey,
    data: T,
) -> ProgramResult {
    let (_, bump) = data.pda(program_id);
    let seeds = data.seeds();
    invoke_signed(
        &system_instruction::create_account(
            payer.key(),
            new_account.key(),
            new_account.required_rent()?,
            new_account.size(),
            program_id,
        ),
        &[payer.into(), new_account.clone().into(), *system_program],
        &[&seeds, &[&[bump]]],
    )?;
    data.serialize(&mut &mut new_account.data.borrow_mut()[..])?;
    Ok(())
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
