#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use std::cell::RefMut;

use anchor_lang::prelude::*;

declare_id!("9o7yLF4venANccENZLow68fEMygUeetXXbjPpuKxYMKm");

#[doc(hidden)]
pub mod errors;
#[doc(hidden)]
pub mod instructions;
#[doc(hidden)]
pub mod states;

use anchor_spl::token_2022::spl_token_2022::{
    extension::{transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut, PodStateWithExtensionsMut},
    pod::PodAccount,
};
pub use instructions::*;
pub use states::*;
pub use errors::ErrorCode;

#[program]
pub mod transfer_hook_dispatcher {
    use spl_transfer_hook_interface::instruction::TransferHookInstruction;

    use super::*;

    pub fn initialize_global_dispatcher_config(
        ctx: Context<InitializeGlobalDispatcherConfigCtx>,
        authority: Pubkey,
    ) -> Result<()> {
        initialize_global_dispatcher_config::handler(ctx, authority)
    }

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaListCtx>,
    ) -> Result<()> {
        initialize_extra_account_meta_list::handler(ctx)
    }

    pub fn add_allowed_hook_program(
        ctx: Context<AddAllowedHookProgramCtx>,
        hook_program: Pubkey,
    ) -> Result<()> {
        add_allowed_hook_program::handler(ctx, hook_program)
    }

    pub fn remove_allowed_hook_program(
        ctx: Context<RemoveAllowedHookProgramCtx>,
        hook_program: Pubkey,
    ) -> Result<()> {
        remove_allowed_hook_program::handler(ctx, hook_program)
    }

    pub fn register_hook_program(
        ctx: Context<RegisterHookProgramCtx>,
        hook_program: Pubkey,
    ) -> Result<()> {
        register_hook_program::handler(ctx, hook_program)
    }

    pub fn unregister_hook_program(
        ctx: Context<UnregisterHookProgramCtx>,
        hook_program: Pubkey,
    ) -> Result<()> {
        unregister_hook_program::handler(ctx, hook_program)
    }

    pub fn on_transfer<'info>(
        ctx: Context<'_, '_, '_, 'info, OnTransferCtx<'info>>,
        amount: u64,
    ) -> Result<()> {
        assert_is_transferring(&ctx)?;
        on_transfer::handler(ctx, amount)
    }

    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data)?;
        // match instruction discriminator to transfer hook interface execute instruction
        // token2022 program CPIs this instruction on token transfer
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();
                // invoke custom transfer hook instruction on our program
                __private::__global::on_transfer(program_id, accounts, &amount_bytes)
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
    }
}

fn assert_is_transferring(ctx: &Context<OnTransferCtx>) -> Result<()> {
    let source_token_info = ctx.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
    if !bool::from(account_extension.transferring) {
        return err!(ErrorCode::IsNotCurrentlyTransferring);
    }
    Ok(())
}
