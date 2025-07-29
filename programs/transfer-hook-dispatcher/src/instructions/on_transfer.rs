use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::{
    token_interface::{Mint, TokenAccount},
};
use spl_transfer_hook_interface::instruction;
use crate::DispatcherAccount;
use crate::errors::ErrorCode;

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

    /// CHECK: source token account owner (could be SystemAccount or PDA)
    pub owner: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaList PDA for this mint
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"dispatcher"],
        bump
    )]
    pub dispatcher_account: Account<'info, DispatcherAccount>,
}

pub fn handler(
    ctx: Context<OnTransfer>,
    amount: u64,
) -> Result<()> {
    let dispatcher: &Account<'_, DispatcherAccount> = &ctx.accounts.dispatcher_account;
    let mut all_accounts = vec![
        ctx.accounts.source_token.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.destination_token.to_account_info(),
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.extra_account_meta_list.to_account_info(),
        ctx.accounts.dispatcher_account.to_account_info()
    ];
    let owned_remaining_accounts = ctx.remaining_accounts
        .cloned()
        .collect::<Vec<_>>();
    all_accounts.extend(owned_remaining_accounts);
    // Dispatch to all registered hook programs
    for hook_program in dispatcher.hook_programs.iter() {
        let ix = instruction::execute(
            hook_program,                          // hook program id
            &ctx.accounts.source_token.key(),                 // source amount
            &ctx.accounts.mint.key(),      // mint
            &ctx.accounts.destination_token.key(), // destination
            &ctx.accounts.owner.key(), // owner
            amount, // amount
        );
        // Invoke the hook program with the provided instruction
        invoke(&ix, &all_accounts)?
    };
    Ok(())
}
