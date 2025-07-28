use anchor_lang::prelude::*;
use crate::states::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UpdateWhitelistAddresses<'info> {
    #[account(mut, has_one = authority)]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateWhitelistAddresses>,
    new_addresses: Vec<Pubkey>,
) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    // Optional: check duplicates
    let mut sorted = new_addresses.clone();
    sorted.sort();
    sorted.dedup();

    if sorted.len() != new_addresses.len() {
        return Err(ErrorCode::DuplicateAddressInWhitelist.into());
    }

    whitelist.addresses = new_addresses;
    Ok(())
}
