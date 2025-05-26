#![no_std]
#![allow(unexpected_cfgs)]
extern crate alloc;

pub mod state;
pub mod instructions;
pub use instructions::*;
pub mod utils;

use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, entrypoint, nostd_panic_handler, ProgramResult, program_error::ProgramError};
use pinocchio_pubkey;

entrypoint!(process_instruction);
nostd_panic_handler!();

pub const ID: Pubkey = pinocchio_pubkey::pubkey!("GgmS3qXsm1SUCKhwrBLbKUoXPNwadQ892TTi6ZM2Um7H");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((CreatePool::DISCRIMINATOR, data)) => CreatePool::try_from((data, accounts))?.process(),
        Some((CreateVault::DISCRIMINATOR, data)) => CreateVault::try_from((data, accounts))?.process(),
        Some((JoinPool::DISCRIMINATOR, data)) => JoinPool::try_from((data, accounts))?.process(),
        Some((Refresh::DISCRIMINATOR, data)) => Refresh::try_from((data, accounts))?.process(),
        Some((ContributeWithVote::DISCRIMINATOR, data)) => ContributeWithVote::try_from((data, accounts))?.process(),
        Some((AcceptParticipant::DISCRIMINATOR, data)) => AcceptParticipant::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
