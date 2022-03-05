use anchor_lang::*;
use solana_program::program_error::ProgramError;
use crate::thiserror::Error;


#[derive(Error, Debug, Copy, Clone)]
pub enum TheWheelError {
    #[error("TheWheel PDA Account given to instruction is wrong one")]
    InvalidTheWheelAccount,

    #[error("Game PDA Account given to instruction is wrong one")]
    InvalidGameAccount,

    #[error("Player PDA Account given to instruction is wrong one")]
    InvalidPlayerAccount,

    #[error("Invalid argument")]
    InvalidInput,

    #[error("Session not valid")]
    SessionNotValid,

    #[error("Session not available")]
    SessionNotAvailable,

    #[error("Player already in pending list")]
    PlayerInPendingList,

    //6
    #[error("Error with pending list")]
    ErrorOnPendingList,

    #[error("Insufficient deposit on player Account")]
    InsufficientDeposit,

    #[error("Max player limit is reached")]
    MaxPlayerLimitReach,

    #[error("Limit time not reach")]
    GameStillValid,

    #[error("Limit time too high")]
    LaunchingDateTooHigh
}

impl From<TheWheelError> for ProgramError {
    fn from(e: TheWheelError) -> Self {
        return ProgramError::Custom(e as u32);
    }
}

    