use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Access Denied")]
    AccessDenied,
    #[msg("Invalid withdraw amount")]
    InvalidAmount,
    #[msg("There is no enuough sol in vault.")]
    NoEnoughSol,
    #[msg("This presale is not finished yet.")]
    PresaleNotFinished,
    #[msg("This presale is not cancelled.")]
    PresaleNotCancelled,
    #[msg("This presale is not started yet.")]
    PresaleNotStarted,
    #[msg("This presale is already ended.")]
    PresaleAlreadyEnded,
    #[msg("There is no enough token in this presale.")]
    PresaleNotEnoughToken,
    #[msg("User does not have any claimable tokens.")]
    NoClaimableToken,
    #[msg("Wrong Time Period.")]
    WrongTimePeriod,
    #[msg("You are not in white list.")]
    NotInWhiteList,
}
