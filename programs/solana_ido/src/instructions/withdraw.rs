use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
    pub admin_account: Box<Account<'info, AdminAccount>>,
    //the authority allowed to transfer from token_from
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[access_control(is_admin(&ctx.accounts.admin_account, &ctx.accounts.admin))]
#[access_control(is_finalized(&ctx.accounts.presale_account))]
pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    let amount = **ctx.accounts.escrow_account.lamports.borrow();
    **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.admin.try_borrow_mut_lamports()? += amount;
    ctx.accounts.presale_account.total_sol_amount = 0;
    Ok(())
}
