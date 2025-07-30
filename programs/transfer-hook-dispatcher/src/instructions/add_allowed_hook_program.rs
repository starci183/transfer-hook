use anchor_lang::prelude::*;
use crate::{GlobalDispatcherConfigAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddAllowedHookProgramCtx<'info> {
    pub authority: Signer<'info>, // admin check
    // The global config is where allowed hooks live
    #[account(mut, has_one = authority)]
    pub global_dispatcher_config_account: Account<'info, GlobalDispatcherConfigAccount>,
}

pub fn handler(
    ctx: Context<AddAllowedHookProgramCtx>,
    hook_program: Pubkey,
) -> Result<()> {
    let global_cfg = &mut ctx.accounts.global_dispatcher_config_account;

    // redundant check (Anchor's has_one already enforces it), but OK
    if global_cfg.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }

    // enforce max length
    if global_cfg.allowed_hook_programs.len() >= 20 {
        return err!(ErrorCode::HookLengthExceeded);
    }

    // prevent duplicates
    if global_cfg.allowed_hook_programs.contains(&hook_program) {
        return err!(ErrorCode::HookAlreadyExists);
    }

    // add hook to global whitelist
    global_cfg.allowed_hook_programs.push(hook_program);

    Ok(())
}
