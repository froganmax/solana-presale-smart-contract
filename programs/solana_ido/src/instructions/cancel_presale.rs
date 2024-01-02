use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct CancelPresale<'info> {
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
pub fn handler(ctx: Context<CancelPresale>) -> Result<()> {
    ctx.accounts.presale_account.is_cancelled = 1;

    emit!(PresaleCancelled {
        total_amount: ctx.accounts.presale_account.total_sol_amount,
        time_stamp: Clock::get().unwrap().unix_timestamp as u32
    });
    Ok(())
}
