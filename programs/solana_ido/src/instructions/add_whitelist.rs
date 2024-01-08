use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct AddWhitelist<'info> {
    #[account(
        mut,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(
        init,
        payer = admin,
        seeds = [ USER_ACCOUNT_SEED.as_bytes(), authority.key().as_ref() ],
        bump,
        space = USER_ACCOUNT_SIZE
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// CHECK
    #[account(mut)]
    pub authority: AccountInfo<'info>, // user wallet address
    #[account(mut)]
    pub admin_account: Box<Account<'info, AdminAccount>>,
    //the authority allowed to transfer from token_from
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[access_control(is_admin(&ctx.accounts.admin_account, &ctx.accounts.admin))]
pub fn handler(ctx: Context<AddWhitelist>) -> Result<()> {
    ctx.accounts.user_account.is_whitelisted = true;
    ctx.accounts.user_account.user_buy_amount = 0;
    ctx.accounts.user_account.user_sol_contributed = 0;
    ctx.accounts.user_account.is_claimed = 0;

    ctx.accounts.presale_account.total_whitelisted_wallets += 1;

    Ok(())
}
