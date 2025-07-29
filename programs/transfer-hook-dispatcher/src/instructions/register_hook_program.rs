use anchor_lang::prelude::*;
use crate::{DispatcherAccount, GlobalDispatcherConfigAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RegisterHookProgram<'info> {
    #[account(mut, has_one = authority)]
    pub dispatcher_account: Account<'info, DispatcherAccount>,

    // read-only check against global whitelist
    pub global_dispatcher_config_account: Account<'info, GlobalDispatcherConfigAccount>,

    /// authority that owns this dispatcher (not global admin)
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<RegisterHookProgram>,
    hook_program: Pubkey,
) -> Result<()> {
    let dispatcher = &mut ctx.accounts.dispatcher_account;
    let global_cfg = &ctx.accounts.global_dispatcher_config_account;

    // 1. authority check (redundant because of has_one but safe)
    if dispatcher.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }

    // 2. hook must be in global whitelist
    if !global_cfg.allowed_hook_programs.contains(&hook_program) {
        return err!(ErrorCode::HookNotAllowed);
    }

    // 3. prevent duplicates in active list
    if dispatcher.hook_programs.contains(&hook_program) {
        return err!(ErrorCode::HookAlreadyExists);
    }

    // 4. enforce max limit
    if dispatcher.hook_programs.len() >= 20 {
        return err!(ErrorCode::HookLengthExceeded);
    }

    // 5. add hook to active list
    dispatcher.hook_programs.push(hook_program);

    Ok(())
}
