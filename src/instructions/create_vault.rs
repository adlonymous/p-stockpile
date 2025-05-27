use pinocchio::{
    account_info::{AccountInfo},
    program_error::ProgramError,
    ProgramResult,
    seeds,
    instruction::{Signer}
};
use pinocchio::sysvars::rent::Rent;
use pinocchio_system::instructions::CreateAccount;
use crate::{
    state::{Vault, VaultAccounts},
};

pub struct CreateVault<'a> {
    pub accounts: VaultAccounts<'a>,
    pub vault: &'a Vault<'a>,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CreateVault<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = VaultAccounts::try_from(accounts)?;
        let vault: &Vault = &Vault::try_from(data)?;

        Ok(Self { accounts, vault })
    }
}

impl<'a> CreateVault<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
    pub fn process(&self) -> ProgramResult {
        let signer_seeds = seeds!(
            Vault::SEED_PREFIX.as_bytes(),
            self.accounts.payer.key().as_ref(),
            &[self.vault.bump]
        );

        let signer = Signer::from(&signer_seeds);

        CreateAccount {
           from: self.accounts.payer,
            to: self.accounts.vault_account,
            lamports: Rent::from_account_info(self.accounts.vault_account)?.minimum_balance(size_of::<Vault>()),
            space: size_of::<Vault>() as u64,
            owner: &self.vault.authority,
        }
            .invoke_signed(&[signer])?;

        Ok(())
    }
}