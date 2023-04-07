mod processor;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::entrypoint;

use processor::{process_instruction, JoeTokenV1, JoeTokenV2, JoeTokenV3};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize, ShankInstruction)]
enum MyInstruction {
    #[account(
        0,
        writable,
        name = "new_account",
        desc = "The new account representing the interface-adherent token PDA."
    )]
    #[account(1, writable, name = "payer", desc = "Fee payer")]
    #[account(2, name = "system_program", desc = "The System Program")]
    CreateTokenV1(JoeTokenV1),
    #[account(
        0,
        writable,
        name = "new_account",
        desc = "The new account representing the interface-adherent token PDA."
    )]
    #[account(1, writable, name = "payer", desc = "Fee payer")]
    #[account(2, name = "system_program", desc = "The System Program")]
    CreateTokenV2(JoeTokenV2),
    #[account(
        0,
        writable,
        name = "new_account",
        desc = "The new account representing the interface-adherent token PDA."
    )]
    #[account(1, writable, name = "payer", desc = "Fee payer")]
    #[account(2, name = "system_program", desc = "The System Program")]
    CreateTokenV3(JoeTokenV3),
    #[account(
        0,
        writable,
        name = "account",
        desc = "The account representing the interface-adherent token PDA."
    )]
    ReadTokenV1,
    #[account(
        0,
        writable,
        name = "account",
        desc = "The account representing the interface-adherent token PDA."
    )]
    ReadTokenV2,
    #[account(
        0,
        writable,
        name = "account",
        desc = "The account representing the interface-adherent token PDA."
    )]
    ReadTokenV3,
}
