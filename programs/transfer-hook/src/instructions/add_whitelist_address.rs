use anchor_lang::prelude::*;
use crate::states::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddWhitelistAddress<'info> {
    /// CHECK: Authority is a signer, no additional checks needed
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<AddWhitelistAddress>, new_address: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    // Check if address is already whitelisted
    if whitelist.addresses.contains(&new_address) {
        return Err(ErrorCode::AddressAlreadyWhitelisted.into());
    }

    whitelist.addresses.push(new_address);
    Ok(())
}