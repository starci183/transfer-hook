    use anchor_lang::prelude::*;
    use crate::{DispatcherAccount, GlobalDispatcherConfigAccount, HookEntry};
    use crate::errors::ErrorCode;

    #[derive(Accounts)]
    pub struct RegisterHookProgramCtx<'info> {
        #[account(mut, has_one = authority)]
        pub dispatcher_account: Account<'info, DispatcherAccount>,
        // read-only check against global whitelist
        pub global_dispatcher_config_account: Account<'info, GlobalDispatcherConfigAccount>,
        /// authority that owns this dispatcher (not global admin)
        pub authority: Signer<'info>,
    }

    pub fn handler(
        ctx: Context<RegisterHookProgramCtx>,
        hook_program: Pubkey,
        additional_accounts: Vec<Pubkey>,  
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
        if dispatcher.hook_entries.iter().any(|entry| entry.program_id == hook_program) {
            return err!(ErrorCode::HookAlreadyExists);
        }

        // 4. enforce max limit
        if dispatcher.hook_entries.len() >= 20 {
            return err!(ErrorCode::HookLengthExceeded);
        }

        // 5. add hook to active list
        dispatcher.hook_entries.push(
            HookEntry {
                program_id: hook_program,
                additional_accounts: additional_accounts,
        });
        Ok(())
    }
