use {anchor_lang::prelude::*, crate::errors::ErrorCode, crate::state::*};
#[derive(Accounts)]
pub struct RefundToken<'info> {
    /// CHECK: This is vault account.
    #[account(
        mut,
        seeds = [ SOL_VAULT_SEED.as_bytes() ],
        bump,
    )]
    pub escrow_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(mut)]
    pub user_account: Box<Account<'info, UserAccount>>,
    //the authority allowed to refund sol
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[access_control(is_cancelled(&ctx.accounts.presale_account))]
pub fn handler(ctx: Context<RefundToken>) -> Result<()> {
    let vault_amount = **ctx.accounts.escrow_account.lamports.borrow();
    let amount = ctx.accounts.user_account.user_buy_amount;

    if vault_amount < amount {
        return Err(error!(ErrorCode::NoEnoughSol));
    }
    **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.authority.try_borrow_mut_lamports()? += amount;

    ctx.accounts.presale_account.total_sol_amount -= amount;
    ctx.accounts.presale_account.total_participants -= 1;
    Ok(())
}
