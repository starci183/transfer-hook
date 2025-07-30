use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::{ExecuteInstruction};
use crate::states::DispatcherAccount;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaListCtx<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        mut,
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        seeds = [b"dispatcher", mint.key().as_ref()], 
        bump,
        payer = payer,
        space =  8 + (4 + 32 * 20) + 32 // 20 hook programs + 32 authority
    )]
    pub dispatcher_account: Account<'info, DispatcherAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

 
pub fn handler(
    ctx: Context<InitializeExtraAccountMetaListCtx>,
) -> Result<()> {
    let mint = ctx.accounts.mint.key();
    // The `addExtraAccountsToInstruction` JS helper function resolving incorrectly
    let account_metas = vec![ExtraAccountMeta::new_with_seeds(
        &[
            Seed::Literal {
                bytes: "dispatcher".as_bytes().to_vec(),
            },
            Seed::AccountKey { index: 0 }
        ],
        false, // is_signer
        true,  // is_writable
    )?];

    // calculate account size
    let account_size = ExtraAccountMetaList::size_of(account_metas.len())? as u64;
    // calculate minimum required lamports
    let lamports = Rent::get()?.minimum_balance(account_size as usize);

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"extra-account-metas",
        &mint.as_ref(),
        &[ctx.bumps.extra_account_meta_list]
    ]];

    // create ExtraAccountMetaList account
    create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            CreateAccount {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.extra_account_meta_list.to_account_info(),
            },
        ).with_signer(signer_seeds),
        lamports,
        account_size,
        ctx.program_id,
    )?;

    // initialize ExtraAccountMetaList account with extra accounts
    ExtraAccountMetaList::init::<ExecuteInstruction>(
        &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        &account_metas,
    )?;

    // set up the dispatcher account
    let dispatcher_account = &mut ctx.accounts.dispatcher_account;
    dispatcher_account.hook_programs = Vec::new();
    dispatcher_account.authority = ctx.accounts.payer.key();

    Ok(())
}
