use anchor_spl::token::{self, Mint, Token, TokenAccount};
use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(
        address = PRESALE_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    //the token account to withdraw to
    pub token_to: Box<Account<'info, TokenAccount>>,

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
    pub rent: Sysvar<'info, Rent>,
}

#[access_control(is_admin(&ctx.accounts.admin_account, &ctx.accounts.admin))]
#[access_control(is_finalized(&ctx.accounts.presale_account))]
pub fn handler(ctx: Context<WithdrawToken>, nonce_vault: u8) -> Result<()> {
    let token_mint_key = ctx.accounts.token_mint.key();
    let seeds = &[token_mint_key.as_ref(), &[nonce_vault]];
    let signer = &[&seeds[..]];
    let amount = ctx.accounts.token_vault.amount;
    //transfer from vault to admin
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.token_to.to_account_info(),
            authority: ctx.accounts.token_vault.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.presale_account.total_token_amount = 0;
    Ok(())
}
