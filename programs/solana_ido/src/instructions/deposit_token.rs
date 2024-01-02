use anchor_spl::token::{self, Mint, Token, TokenAccount};
use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(
        address = PRESALE_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    //the token account to deposit from
    pub token_from: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ token_mint.key().as_ref() ],
        bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

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
    pub token_program: Program<'info, Token>,
}

#[access_control(is_admin(&ctx.accounts.admin_account, &ctx.accounts.admin))]
pub fn handler(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
    //transfer the admin tokens to the vault
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.token_from.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;

    if ctx.accounts.presale_account.total_sol_amount != 0 {
        ctx.accounts.presale_account.presale_rate =
            (ctx.accounts.presale_account.total_token_amount + amount)
                / ctx.accounts.presale_account.total_sol_amount;
    }
    ctx.accounts.presale_account.total_token_amount += amount;
    Ok(())
}
