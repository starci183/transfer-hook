use anchor_lang::prelude::*;

use crate::GlobalDispatcherConfigAccount;

/// Seeds for the global dispatcher config PDA
pub const GLOBAL_DISPATCHER_SEED: &[u8] = b"global-dispatcher-config";

#[derive(Accounts)]
pub struct InitializeGlobalDispatcherConfigCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [GLOBAL_DISPATCHER_SEED],
        bump,
        payer = payer,
        space = 8  // discriminator
            + (4 + 32 * 20) // allowed_hook_programs: max 20 hooks
            + 32 // authority pubkey
    )]
    pub global_dispatcher_config: Account<'info, GlobalDispatcherConfigAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeGlobalDispatcherConfigCtx>,
    authority: Pubkey,
) -> Result<()> {
    let cfg = &mut ctx.accounts.global_dispatcher_config;
    cfg.allowed_hook_programs = Vec::new();
    cfg.authority = authority;
    Ok(())
}
