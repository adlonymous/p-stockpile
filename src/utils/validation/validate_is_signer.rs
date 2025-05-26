use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
};

pub fn validate_is_signer(
    account: &AccountInfo
) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(ProgramError::InvalidAccountOwner)
    };
    Ok(())
}