// #![feature(build_hasher_simple_hash_one)]

pub mod errors;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("CzvDqDn7oCVwh9fHYDRCVhXqFDtqDxbNfJrk3mYYKKHn");

#[program]
mod token_presale {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        buy_token::handler(ctx, amount)
    }

    pub fn refund_token(ctx: Context<RefundToken>) -> Result<()> {
        refund_token::handler(ctx)
    }

    pub fn claim_token(
        ctx: Context<ClaimToken>,
        nonce_vault: u8,
        token_vault_bump: u8,
        presale_account_bump: u8,
    ) -> Result<()> {
        claim_token::handler(ctx, nonce_vault, token_vault_bump, presale_account_bump)
    }

    pub fn cancel_presale(ctx: Context<CancelPresale>) -> Result<()> {
        cancel_presale::handler(ctx)
    }

    pub fn update_presale_period(
        ctx: Context<UpdatePresalePeriod>,
        price: u64,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        update_presale_period::handler(ctx, price, start_time, end_time)
    }

    // pub fn set_whitelist(ctx: Context<SetWhitelist>, has_whitelist: u8) -> Result<()> {
    //     set_whitelist:: handler(ctx, has_whitelist)
    // }

    pub fn add_whitelist(ctx: Context<AddWhitelist>) -> Result<()> {
        add_whitelist::handler(ctx)
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>) -> Result<()> {
        remove_whitelist::handler(ctx)
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        finalize::handler(ctx)
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        deposit_token::handler(ctx, amount)
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, nonce_vault: u8) -> Result<()> {
        withdraw_token::handler(ctx, nonce_vault)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::handler(ctx)
    }
}
