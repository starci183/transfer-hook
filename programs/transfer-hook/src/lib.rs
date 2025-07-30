use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenAccount},
};

declare_id!("BABvyPc1kHb2xTeV21H4oVwpzc9YdyicBJMNdn4nNKy7");

#[error_code]
pub enum MyError {
    #[msg("The amount is too big")]
    AmountTooBig,
}

#[program]
pub mod transfer_hook {
    use super::*;
    /// Tạo counter PDA cho mint này
    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        ctx.accounts.counter_account.counter = 0;
        Ok(())
    }

    /// Hàm process_transfer được dispatcher gọi
    pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
        if amount > 100_000_000 {
            return err!(MyError::AmountTooBig);
        }
        // ctx.accounts.counter_account.counter += 1;
        // msg!(
        //     "Transfer processed, amount = {}, total transfers = {}",
        //     amount,
        //     ctx.accounts.counter_account.counter
        // );
        Ok(())
    }

    pub fn hello_world(ctx: Context<HelloWorld>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[account]
pub struct CounterAccount {
    pub counter: u64,
}

#[derive(Accounts)]
pub struct HelloWorld {
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"counter"],
        bump,
        payer = payer,
        space = 8 + 8, // 8 discriminator + u64
    )]
    pub counter_account: Account<'info, CounterAccount>,
    pub system_program: Program<'info, System>,
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

    /// CHECK: chủ của source token account
    pub owner: UncheckedAccount<'info>,

    // #[account(
    //     mut,
    //     seeds = [b"counter"],
    //     bump,
    // )]
    // pub counter_account: Account<'info, CounterAccount>,
}
