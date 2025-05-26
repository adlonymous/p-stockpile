use alloc::vec;
use pinocchio::{
    account_info::{AccountInfo},
    ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvars::{rent::Rent,},
    seeds,
    instruction::{Signer}
};
use pinocchio_system::instructions::{CreateAccount};
use jaguar::{JaguarSerialize, JaguarDeserialize, JaguarSerializer, JaguarDeserializer};

use crate::{
    state::Pool,
    utils::validation::validate_is_signer,
};

#[derive(Debug, JaguarSerialize, JaguarDeserialize)]
pub struct CreatePoolArgs<'a> {
    pub pool: &'a Pool,
}

pub fn create_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreatePoolArgs
) -> ProgramResult {

    let [pool_account, payer, system_program, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };
    validate_is_signer(payer)?;

    let rent = Rent::from_account_info(pool_account)?;
    let rent_minimum = rent.minimum_balance(size_of::<Pool>());

    let signer_seeds = seeds!(
        Pool::SEED_PREFIX.as_bytes(),
        payer.key().as_ref(),
        &[args.pool.bump]
    );

    let signer = Signer::from(&signer_seeds);

    CreateAccount {
        from: payer,
        to: pool_account,
        lamports: rent_minimum,
        space: size_of::<Pool>() as u64,
        owner: program_id,
    }
        .invoke_signed(&[signer])?;

    Ok(())
}
