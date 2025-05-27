use heapless::String;
use pinocchio::{
    program_error::ProgramError,
    pubkey::Pubkey,
    account_info::AccountInfo,
};
use crate::utils::validation::validate_is_signer;

#[derive(Default)]
pub enum VaultState<'a> {
    #[default]
    Active,
    Deactivated,
    Closed
}

impl<'a> TryFrom<u8> for VaultState<'a> {
    type Error = ProgramError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Active),
            1 => Ok(Self::Deactivated),
            2 => Ok(Self::Closed),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

pub struct Vault<'a> {
    pub authority: Pubkey,
    pub vault_state: VaultState<'a>,
    pub bump: u8,
    pub name: String<64>
}

#[derive(Debug)]
pub struct VaultAccounts<'a> {
    pub vault_account: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for VaultAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [vault_account, payer, system_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        validate_is_signer(payer)?;

        Ok (Self { vault_account, payer, system_program })
    }
}

impl<'a> TryFrom<&'a [u8]> for Vault<'a> {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<Vault>() {
            return Err(ProgramError::InvalidInstructionData);
        };

        Ok(Self{
            authority: (data[0..32].try_into().unwrap()),
            vault_state: VaultState::Active,
            bump: data[33],
            name: (data[34..98].try_into().unwrap()),
        })
    }
}

impl<'a> Vault<'a> {
    pub const SEED_PREFIX: &'static str = "vault";

    pub fn is_active(&self) -> Result<(), ProgramError> {
        match self.vault_state {
            VaultState::Closed => Err(ProgramError::InvalidArgument),
            VaultState::Active => Ok(()),
            VaultState::Deactivated => Err(ProgramError::InvalidArgument),
        }
    }

}