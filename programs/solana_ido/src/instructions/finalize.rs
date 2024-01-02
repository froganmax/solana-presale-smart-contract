use {anchor_lang::prelude::*, crate::errors::ErrorCode, crate::state::*};

#[derive(Accounts)]
pub struct Finalize<'info> {
    /// CHECK: This is vault account.
    #[account(
        mut,
        seeds = [ SOL_VAULT_SEED.as_bytes() ],
        bump,
    )]
    pub escrow_account: UncheckedAccount<'info>,
    /// CHECK: This is team account.
    #[account(mut)]
    //the team account to withdraw
    pub team_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(mut)]
    pub admin_account: Box<Account<'info, AdminAccount>>,
    //the authority allowed to transfer from token_from
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[access_control(is_admin(&ctx.accounts.admin_account, &ctx.accounts.admin))]
pub fn handler(ctx: Context<Finalize>) -> Result<()> {
    let team_amount = (ctx.accounts.presale_account.team_percent as u64
        * ctx.accounts.presale_account.total_sol_amount
        / 10000) as u64;
    let amount = **ctx.accounts.escrow_account.lamports.borrow();
    if amount < team_amount {
        return Err(error!(ErrorCode::NoEnoughSol));
    }

    **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= team_amount;
    **ctx.accounts.team_account.try_borrow_mut_lamports()? += team_amount;

    ctx.accounts.presale_account.is_finalized = 1;

    emit!(PresaleFinalized {
        total_amount: ctx.accounts.presale_account.total_sol_amount,
        time_stamp: Clock::get().unwrap().unix_timestamp as u32
    });
    Ok(())
}
