use pinocchio::{pubkey::Pubkey, program_error::ProgramError, sysvars::clock::Clock, sysvars::Sysvar, account_info::AccountInfo};
use shank::ShankAccount;
use heapless::String;

//TODO: Try to incorporate Sybil resistance here through a SybilStrategy struct

#[repr(C)]
#[derive(Debug, Clone, PartialEq, ShankAccount)]
pub struct Pool {
    pub start: u64,
    pub end: u64,
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    // pub sybil_strategy: InProgress,
    pub participant_index: u8,
    pub bump: u8,
    pub name: String<64>,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const MAX_NAME_LENGTH: usize = 64;
    pub const MAX_PARTICIPANTS: usize = 255;
    pub const SPACE: usize = Self::MAX_NAME_LENGTH + 8 + 8 + 32 + 32 + 1 + 4 + 1 + 1 + 1;

    pub fn new(
        name: String<64>,
        start: u64,
        end: u64,
        mint: Pubkey,
        admin: Pubkey,
        access: PoolAccess,
        bump: u8,
        clock_info: &AccountInfo
    ) -> Result<Self, ProgramError> {
        if name.as_bytes().len() > Self::MAX_NAME_LENGTH {
            return Err(ProgramError::InvalidArgument);
        }

        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        if timestamp > start {
            return Err(ProgramError::InvalidArgument);
        }

        Ok(Self {
            name,
            start,
            end,
            mint,
            admin,
            pool_state: PoolState::PendingStart,
            pool_access: access,
            participant_index: 0,
            bump
        })
    }

    pub fn is_active(&mut self) -> Result<(), ProgramError> {
        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;

        if timestamp > self.end {
            return Err(ProgramError::InvalidArgument);
            // TODO: Maybe write a custom error for timestamp stuff
        }

        match self.pool_state {
            PoolState::PendingStart => Err(ProgramError::InvalidArgument),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(ProgramError::InvalidArgument),
        }

    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PoolState {
    #[default]
    PendingStart,
    Active,
    Distributed,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PoolAccess {
    Open,
    #[default]
    Manual,
    TokenGated(TokenGateInfo)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TokenGateInfo {
    pub mint: Pubkey,
    pub min_amount: u64,
}


