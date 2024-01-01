pub mod errors;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("C9FnHPykP7m89VirYWgTfV3YDPFULtDeFgwPBAJZABrR");

#[program]
mod token_presale {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize:: handler(ctx)
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        buy_token:: handler(ctx, amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        refund:: handler(ctx)
    }

    pub fn claim_token(ctx: Context<ClaimToken>, nonce_vault:u8) -> Result<()> {
        claim_token:: handler(ctx, nonce_vault)
    }

    pub fn cancel_presale(ctx: Context<CancelPresale>) -> Result<()> {
        cancel_presale:: handler(ctx)
    }

    pub fn update_presale_period(ctx: Context<UpdatePresalePeriod>, start_time: u32, end_time: u32) -> Result<()> {
        update_presale_period:: handler(ctx, start_time, end_time)
    }

    pub fn set_whitelist(ctx: Context<SetWhitelist>, has_whitelist: u8) -> Result<()> {
        set_whitelist:: handler(ctx, has_whitelist)
    }

    pub fn add_whitelist(ctx: Context<AddWhitelist>, addresses: Vec<String>) -> Result<()> {
        add_whitelist:: handler(ctx, addresses)
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, addresses: Vec<String>) -> Result<()> {
        remove_whitelist:: handler(ctx, addresses)
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        finalize:: handler(ctx)
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        deposit_token:: handler(ctx, amount)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, nonce_vault: u8) -> Result<()> {
        withdraw_token:: handler(ctx, nonce_vault)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw:: handler(ctx)
    }
   
}