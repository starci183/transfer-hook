
use std::num::TryFromIntError;
use anchor_lang::prelude::*;
#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Recipient address is not whitelisted")]
    RecipientNotWhitelisted, // 0x1771 (6000)
    #[msg("Exceeded maximum number of transfers")]
    ExceededMaxTransfers, // 0x1772 (6001)
    #[msg("Transfer amount exceeds limit")]
    TransferAmountExceedsLimit, // 0x1773 (6002)
    #[msg("Invalid transfer amount")]
    InvalidTransferAmount, // 0x1774 (6003)
    #[msg("Number cast error")]
    NumberCastError, // 0x1775 (6004)
    #[msg("Invalid account state")]
    InvalidAccountState, // 0x1776 (6005)
    #[msg("Address is already whitelisted")]
    AddressAlreadyWhitelisted, // 0x1777 (6006)
    #[msg("Address is not whitelisted")]
    AddressNotWhitelisted, // 0x1778 (6007)
    #[msg("Duplicate address in whitelist")]
    DuplicateAddressInWhitelist, // 0x1779 (6008)
}

impl From<TryFromIntError> for ErrorCode {
    fn from(_: TryFromIntError) -> Self {
        ErrorCode::NumberCastError
    }
}
