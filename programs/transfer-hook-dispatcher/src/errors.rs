use std::num::TryFromIntError;
use anchor_lang::prelude::*;
#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Hook length exceeds maximum allowed length")]
    HookLengthExceeded = 0x1771, // 0x1771 (6000)
    #[msg("Hook already exists in the whitelist")]
    HookAlreadyExists = 0x1772, // 0x1772 (6001)
    #[msg("Number cast error")]
    NumberCastError = 0x1773, // 0x1773 (6002)
    #[msg("Invalid hook program")]
    InvalidHookProgram = 0x1774, // 0x1774 (6003)
    #[msg("Not authorized to perform this action")]
    Unauthorized = 0x1775, // 0x1775 (6004)
    #[msg("Hook not found in the allowed list")]
    HookNotFound = 0x1776, // 0x1776 (6005)
    #[msg("Hook program is not allowed in the global whitelist")]
    HookNotAllowed = 0x1777, // 0x1777 (6006)
    #[msg("Missing remaining account")]
    MissingRemainingAccount = 0x1778, // 0x1778 (6007)
}

impl From<TryFromIntError> for ErrorCode {
    fn from(_: TryFromIntError) -> Self {
        ErrorCode::NumberCastError
    }
}
