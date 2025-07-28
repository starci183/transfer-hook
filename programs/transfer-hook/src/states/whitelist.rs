use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub addresses: Vec<Pubkey>, // danh sách các address được phép nhận token
}

impl Whitelist {
    // Max 100 addresses: 32 * 100 + 32 (authority)
    pub const MAX_SIZE: usize = 32 + (32 * 100) + 4; // +4 cho prefix vec
}