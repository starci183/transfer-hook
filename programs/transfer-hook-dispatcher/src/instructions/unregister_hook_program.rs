use anchor_lang::prelude::*;
use crate::DispatcherAccount;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UnregisterHookProgramCtx<'info> {
    #[account(mut, has_one = authority)]
    pub dispatcher_account: Account<'info, DispatcherAccount>,

    /// authority that owns this dispatcher
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UnregisterHookProgramCtx>,
    hook_program: Pubkey,
) -> Result<()> {
    let dispatcher = &mut ctx.accounts.dispatcher_account;

    // authority check (redundant due to has_one but safe)
    if dispatcher.authority != ctx.accounts.authority.key() {
        return err!(ErrorCode::Unauthorized);
    }

    // find and remove from active list
    if let Some(pos) = dispatcher.hook_entries.iter().position(|hook_entry| hook_entry.program_id == hook_program) {
        dispatcher.hook_entries.remove(pos);
        Ok(())
    } else {
        // not found
        err!(ErrorCode::HookNotFound)
    }
}   
