use pinocchio::{
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    account_info::AccountInfo
};
use heapless::{String, Vec};
use crate::utils::validation::validate_is_signer;

// TODO: Add Sybil protection and Token gated pools

#[derive(Default)]
pub enum PoolState {
    #[default]
    PendingStart,
    Active,
    Distributed,
}

#[derive(Default)]
pub enum PoolAccess {
    Open,
    #[default]
    Manual,
}

impl TryFrom<u8> for PoolAccess {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Open),
            1 => Ok(Self::Manual),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl TryFrom<u8> for PoolState {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PendingStart),
            1 => Ok(Self::Active),
            2 => Ok(Self::Distributed),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

pub struct Pool<'a> {
    pub start: u64,
    pub end: u64,
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub participant_index: u8,
    pub bump: u8,
    pub name: String<32>,
}

#[derive(Debug)]
pub struct PoolAccounts<'a> {
    pub pool_account: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for PoolAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [pool_account, payer, system_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        validate_is_signer(payer)?;

        Ok (Self { pool_account, payer, system_program })
    }
}

impl<'a> TryFrom<&'a [u8]> for Pool<'a> {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<Pool>() {
            return Err(ProgramError::InvalidInstructionData);
        };

        if data.eq(&[0; size_of::<Pool>()]) {
            return Err(ProgramError::InvalidInstructionData);
        }

        if data[149] {
            return Err(ProgramError::InvalidInstructionData);
        }

        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        let start_time = u64::from_le_bytes(data[0..8].try_into().unwrap());
        if timestamp > start_time {
            return Err(ProgramError::InvalidArgument);
        }

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&data[84..148]);
        let pool_name = String::from_utf8(bytes).unwrap();

        Ok(Self {
            start: start_time,
            end: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            mint: (data[16..48].try_into().unwrap()),
            admin: (data[48..80].try_into().unwrap()),
            pool_state: PoolState::try_from(data[80])?,
            pool_access: PoolAccess::try_from(data[81])?,
            participant_index: data[82],
            bump: data[83],
            name: pool_name,
        })
    }

}

impl<'a> Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub fn is_active(&self) -> Result<(), ProgramError> {
        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;

        if timestamp > self.end {
            return Err(ProgramError::InvalidArgument);
        }

        match self.pool_state {
            PoolState::PendingStart => Err(ProgramError::InvalidArgument),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProgramError::InvalidArgument),
        }
    }
}



