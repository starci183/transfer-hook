use anchor_lang::prelude::*;

/// Local dispatcher account (per mint or instance).
/// Each DispatcherAccount is owned by an authority and
/// contains the list of active hook programs.
#[account]
pub struct DispatcherAccount {
    /// List of registered (active) hook programs for this dispatcher.
    /// Each Pubkey represents a hook program.
    pub hook_entries: Vec<HookEntry>,

    /// The authority that can manage (add/remove) hook_programs.
    /// Usually set to the payer at initialization.
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct HookEntry {
    pub program_id: Pubkey,
    pub additional_accounts: Vec<AccountInfoLike>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AccountInfoLike {
    pub key: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// Global configuration account shared across the entire program.
/// Stores a whitelist of allowed hook programs that can be registered
/// into each DispatcherAccount. Ensures only whitelisted hooks are used.
#[account]
pub struct GlobalDispatcherConfigAccount {
    /// Global whitelist of allowed hook programs.
    /// Only Pubkeys in this list can be registered in DispatcherAccount.
    pub allowed_hook_programs: Vec<Pubkey>,

    /// The authority that can manage the global whitelist.
    /// Typically the program admin/deployer.
    pub authority: Pubkey,
}