use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::{
    token_interface::{Mint, TokenAccount},
};
use spl_transfer_hook_interface::instruction;
use crate::DispatcherAccount;

#[derive(Accounts)]
pub struct OnTransferCtx<'info> {
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

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, OnTransferCtx<'info>>,
    amount: u64,
) -> Result<()> {
    // Phase 1: clone hook_programs first (early release of borrow)
    let hooks: Vec<Pubkey> = ctx.accounts.dispatcher_account.hook_programs.clone();
    // Phase 2: collect all account infos with proper lifetime handling
    let mut all_accounts = Vec::with_capacity(6 + ctx.remaining_accounts.len());
    // Add main accounts
    all_accounts.push(ctx.accounts.dispatcher_account.to_account_info());
    all_accounts.push(ctx.accounts.source_token.to_account_info());
    all_accounts.push(ctx.accounts.mint.to_account_info());
    all_accounts.push(ctx.accounts.destination_token.to_account_info());
    all_accounts.push(ctx.accounts.owner.to_account_info());
    all_accounts.push(ctx.accounts.extra_account_meta_list.to_account_info());
    // Add remaining accounts with explicit lifetime handling
    all_accounts.extend(
        ctx.remaining_accounts.iter().map(|acc| acc.to_account_info())
    );
    // Phase 3: invoke all hook_programs
    for hook_program in hooks.iter() {
        let hook_instruction = instruction::execute(
            hook_program,
            &ctx.accounts.source_token.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.destination_token.key(),
            &ctx.accounts.owner.key(),
            amount,
        );
        invoke(&hook_instruction, &all_accounts)?;
    }
    Ok(())
}