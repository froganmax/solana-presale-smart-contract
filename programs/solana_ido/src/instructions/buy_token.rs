use anchor_lang::solana_program::{program::invoke, system_instruction};
use {anchor_lang::prelude::*, crate::errors::ErrorCode, crate::state::*};

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(
        mut,
        seeds = [ SOL_VAULT_SEED.as_bytes() ],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    //the authority allowed to transfer sol
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[access_control(is_presale_live(&ctx.accounts.presale_account))]
pub fn handler(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
    if ctx.accounts.presale_account.has_whitelist > 0 {
        match ctx
            .accounts
            .presale_account
            .white_list
            .iter()
            .position(|og| *og == ctx.accounts.authority.key.key().to_string())
        {
            Some(_index) => {}
            None => {
                return Err(error!(ErrorCode::NotInWhiteList));
            }
        }
    }
    if **ctx.accounts.authority.lamports.borrow() < amount {
        return Err(error!(ErrorCode::NoEnoughSol));
    }

    // transfer bet amount to escrow account
    invoke(
        &system_instruction::transfer(
            ctx.accounts.authority.key,
            ctx.accounts.escrow_account.key,
            amount,
        ),
        &[
            ctx.accounts.authority.to_account_info().clone(),
            ctx.accounts.escrow_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    if ctx.accounts.user_account.user_buy_amount == 0 {
        ctx.accounts.presale_account.total_participants += 1;
    }

    ctx.accounts.presale_account.presale_rate = ctx.accounts.presale_account.total_token_amount
        / (ctx.accounts.presale_account.total_sol_amount + amount);
    ctx.accounts.presale_account.total_sol_amount += amount;
    ctx.accounts.user_account.user_buy_amount += amount;

    emit!(UserBought {
        user: *ctx.accounts.authority.key,
        amount: amount,
        time_stamp: Clock::get().unwrap().unix_timestamp as u32
    });
    Ok(())
}
