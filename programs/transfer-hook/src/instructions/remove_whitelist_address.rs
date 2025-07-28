use anchor_lang::prelude::*;
use crate::states::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RemoveWhitelistAddress<'info> {
    #[account(mut, has_one = authority)]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<RemoveWhitelistAddress>,
    address_to_remove: Pubkey,
) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    // Find index of address to remove
    if let Some(index) = whitelist.addresses.iter().position(|x| x == &address_to_remove) {
        whitelist.addresses.swap_remove(index); // Efficient remove without preserving order
        Ok(())
    } else {
        Err(ErrorCode::AddressNotWhitelisted.into())
    }
}