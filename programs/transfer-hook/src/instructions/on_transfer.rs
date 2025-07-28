use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::Whitelist;
use anchor_spl::token_interface::{ TokenAccount, Mint };

#[derive(Accounts)]
pub struct OnTransfer<'info> {
    #[account(
        token::mint = mint, 
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"counter"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<OnTransfer>, _amount: u64) -> Result<()> {
    let to = ctx.accounts.destination_token.key();
    let whitelist = &ctx.accounts.whitelist;

    let is_whitelisted = whitelist.addresses.iter().any(|addr| addr == &to);
    if !is_whitelisted {
        return Err(ErrorCode::RecipientNotWhitelisted.into());
    }

    Ok(())
}