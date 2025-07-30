use anchor_lang::prelude::*;
use crate::GlobalDispatcherConfigAccount;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RemoveAllowedHookProgramCtx<'info> {
    #[account(mut, has_one = authority)]
    pub global_dispatcher_config_account: Account<'info, GlobalDispatcherConfigAccount>,

    pub authority: Signer<'info>, // admin must sign
}

pub fn handler(
    ctx: Context<RemoveAllowedHookProgramCtx>,
    hook_program: Pubkey,
) -> Result<()> {
    let global_cfg = &mut ctx.accounts.global_dispatcher_config_account;

    // redundant check, already enforced by has_one
    if global_cfg.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }
    
    // find and remove
    if let Some(pos) = global_cfg
        .allowed_hook_programs
        .iter()
        .position(|x| *x == hook_program)
    {
        global_cfg.allowed_hook_programs.remove(pos);
        Ok(())
    } else {
        err!(ErrorCode::HookNotFound)
    }
}
