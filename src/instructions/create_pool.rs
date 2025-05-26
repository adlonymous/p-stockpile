use pinocchio::{
    account_info::{AccountInfo},
    ProgramResult,
    program_error::ProgramError,
    sysvars::{rent::Rent,},
    seeds,
    instruction::{Signer}
};
use pinocchio_system::instructions::{CreateAccount};
use crate::{
    state::{Pool, PoolAccounts},
};

#[derive(Debug)]
pub struct CreatePoolArgs<'a> {
    pub accounts: PoolAccounts<'a>,
    pub pool: &'a Pool<'a>,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CreatePoolArgs<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = PoolAccounts::try_from(accounts)?;
        let pool: &Pool = &Pool::try_from(data)?;

        Ok(Self { accounts, pool })
    }
}

impl<'a> CreatePoolArgs<'a> {
    pub fn process(&self) -> ProgramResult {

        let signer_seeds = seeds!(
            Pool::SEED_PREFIX.as_bytes(),
            self.accounts.payer.key().as_ref(),
            &[self.pool.bump]
        );

        let signer = Signer::from(&signer_seeds);

        CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.pool_account,
            lamports: Rent::from_account_info(self.accounts.pool_account)?.minimum_balance(size_of::<Pool>()),
            space: size_of::<Pool>() as u64,
            owner: &self.pool.admin,
        }
            .invoke_signed(&[signer])?;

        Ok(())
    }
}
