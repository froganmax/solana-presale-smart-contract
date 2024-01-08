use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(
        mut,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(
        mut,
        seeds = [ USER_ACCOUNT_SEED.as_bytes(), authority.key().as_ref() ],
        bump,
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
pub fn handler(ctx: Context<RemoveWhitelist>) -> Result<()> {
    ctx.accounts.user_account.is_whitelisted = false;
    ctx.accounts.presale_account.total_whitelisted_wallets -= 1;
    Ok(())
}
