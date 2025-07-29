#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("equy7LpifBUF1Kd13MKvuUXjdw9Wt8uaqH1FSXp72vw");

#[doc(hidden)]
pub mod states;
#[doc(hidden)]
pub mod instructions;
#[doc(hidden)]
pub mod errors;

pub use states::*;
pub use instructions::*;

#[program]
pub mod transfer_hook_dispatcher {
    use super::*;

    pub fn initialize_global_dispatcher_config(
        ctx: Context<InitializeGlobalDispatcherConfigCtx>,
        authority: Pubkey,
    ) -> Result<()> {
        initialize_global_dispatcher_config::handler(ctx, authority)
    }

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        initialize_extra_account_meta_list::handler(ctx)
    }

    pub fn add_allowed_hook_program(
        ctx: Context<AddAllowedHookProgram>,
        hook_program: Pubkey,
    ) -> Result<()> {
        add_allowed_hook_program::handler(ctx, hook_program)
    }

    pub fn remove_allowed_hook_program(
        ctx: Context<RemoveAllowedHookProgram>,
        hook_program: Pubkey,
    ) -> Result<()> {
        remove_allowed_hook_program::handler(ctx, hook_program)
    }

    pub fn register_hook_program(
        ctx: Context<RegisterHookProgram>,
        hook_program: Pubkey,
    ) -> Result<()> {
        register_hook_program::handler(ctx, hook_program)
    }

    pub fn unregister_hook_program(
        ctx: Context<UnregisterHookProgram>,
        hook_program: Pubkey,
    ) -> Result<()> {
        unregister_hook_program::handler(ctx, hook_program)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
