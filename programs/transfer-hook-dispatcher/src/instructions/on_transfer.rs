use anchor_lang::{
    prelude::*, solana_program::{
        entrypoint_deprecated::ProgramResult, instruction::Instruction, program::invoke
    }
};
use anchor_spl::{
    token_interface::{Mint, TokenAccount},
};
use crate::{DispatcherAccount, HookEntry};
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
        seeds = [b"dispatcher", mint.key().as_ref()],
        bump
    )]
    pub dispatcher_account: Account<'info, DispatcherAccount>,
}


#[derive(Accounts)]
pub struct Execute<'info> {
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

    /// CHECK: owner of the source token account
    pub owner: UncheckedAccount<'info>,
}

pub fn handler<'info>(
    ctx: Context<OnTransferCtx>,
    amount: u64,
) -> Result<()> {
    let hook_programs = ctx.accounts.dispatcher_account.hook_entries.iter()
        .map(|entry| entry.program_id)
        .collect::<Vec<Pubkey>>();
    //Phase 3: invoke all hook_programs
    for hook_program in hook_programs.iter() {
        let additional_accounts = ctx
            .accounts
            .dispatcher_account
            .hook_entries
            .iter()
            .find(|hook_entry| hook_entry.program_id == *hook_program)
            .cloned()
            .unwrap()
            .additional_accounts
            .iter();
        msg!("Invoking hook program: {} with {} accounts, remaining accounts {}", hook_program, additional_accounts.len(), ctx.remaining_accounts.len());
        // Call the hook program with the collected accounts and amount
        let _ = execute(accounts.clone(), *hook_program, amount);
    }
    Ok(())
}

pub fn execute<'info>(
    accounts: Vec<AccountInfo<'info>>,
    hook_program: Pubkey,
    amount: u64,
) -> ProgramResult {
    // 1. Discriminator
    let mut data = get_function_hash("global", "execute").to_vec();
    // 2. Serialize amount (little endian)
    data.extend_from_slice(&amount.to_le_bytes());
    // 3. Build instruction
    let instruction = Instruction {
        program_id: hook_program,
        accounts: accounts
            .iter()
            .map(|account| AccountMeta::new_readonly(account.key(), account.is_signer))
            .collect::<Vec<AccountMeta>>(),
        data,
    };
    // 4. Invoke
    invoke(&instruction, accounts.as_slice())
}

pub fn get_function_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}