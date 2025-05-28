use pinocchio::{
    sysvars::{Sysvar, clock::Clock},
    pubkey::Pubkey,
    program_error::ProgramError,
};


#[derive(Default)]
pub enum AcceptanceStatus {
    #[default]
    Pending,
    Accepted,
    Denied,
}

impl<'a> TryFrom<u8> for AcceptanceStatus {
    type Error = ProgramError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pending),
            1 => Ok(Self::Accepted),
            2 => Ok(Self::Denied),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

pub struct Participant<'a> {
    pub pool_id: Pubkey,
    pub vault_id: Pubkey,
    pub timestamp: u64,
    pub table_index: u8,
    pub status: AcceptanceStatus,
    pub bump: u8
};

impl<'a> TryFrom<&'a [u8]> for Participant<'a> {
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let current = Clock::get().unwrap();
        let time = current.unix_timestamp as u64;

        Ok(Self{
            pool_id: (value[0..32].try_into().unwrap()),ß
            vault_id: (value[32..64].try_into().unwrap()),
            timestamp: time,
            table_index: 0,
            status: AcceptanceStatus::try_from(value[64])?,
            bump: value[66]
        })
    }
}

