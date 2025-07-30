use anchor_lang::{
    prelude::*, 
    solana_program::{
        program::{invoke},
        instruction::{Instruction},
    },
};
use anchor_spl::{
    token_interface::{Mint, TokenAccount},
};
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
        seeds = [b"dispatcher", mint.key().as_ref()],
        bump
    )]
    pub dispatcher_account: Account<'info, DispatcherAccount>,
}


#[derive(Accounts)]
pub struct ProcessTransfer<'info> {
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

    /// CHECK: chủ của source token account
    pub owner: UncheckedAccount<'info>,
}

pub fn handler<'info>(
    ctx: Context<OnTransferCtx>,
    amount: u64,
) -> Result<()> {
    // Phase 1: clone hook_programs first (early release of borrow)
    let hooks: Vec<Pubkey> = ctx.accounts.dispatcher_account.hook_programs.clone();
    // Phase 2: collect all account infos with proper lifetime handling
    let accounts = vec![
        ctx.accounts.source_token.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.destination_token.to_account_info(),
        ctx.accounts.owner.to_account_info(),
    ];
    msg!("on_transfer: amount = {}, hooks = {:?}", amount, hooks);
    //Phase 3: invoke all hook_programs
    for hook_program in hooks.iter() {
        msg!("Invoking hook program: {}", hook_program);
        // Call the hook program with the collected accounts and amount
        call_hook(*hook_program)?;
    }
    Ok(())
}

// process_transfer function to handle the transfer logic
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum HookInstruction {
    Execute { amount: u64 },
    HelloWorld,
}

// pub fn call_hook<'info>(
//     hook_program: Pubkey,
//     accounts: &[AccountInfo<'info>],
//     amount: u64,
// ) -> Result<()> {
//     // let data = HookInstruction::HelloWorld { } 
//     //     .try_to_vec()
//     //     .map_err(|_| ProgramError::InvalidInstructionData)?;

//     // let instruction = Instruction {
//     //     program_id: hook_program,
//     //     accounts: vec![
//     //         // AccountMeta::new_readonly(*accounts[0].key, false), // source
//     //         // AccountMeta::new_readonly(*accounts[1].key, false), // mint
//     //         // AccountMeta::new_readonly(*accounts[2].key, false), // destination
//     //         // AccountMeta::new_readonly(*accounts[3].key, false), // owner
//     //     ],
//     //     data,
//     // };
//     // invoke(&instruction, accounts)
//     Ok(())
// }

pub fn call_hook<'info>(
    hook_program: Pubkey,
) -> Result<()> {
    // serialize the amount into bytes
    let data = get_function_hash("global", "hello_world").to_vec();
    // Not needed to pass accounts here, as we are not using them in the hook
    let instruction = Instruction {
        program_id: hook_program,
        accounts: vec![],
        data,
    };
    // Invoke to the hook program
    invoke(&instruction, &[])?; 

    Ok(())
}

pub fn get_function_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}