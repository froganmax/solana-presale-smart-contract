use anchor_spl::token::{self, Mint, Token, TokenAccount};
use {anchor_lang::prelude::*, crate::errors::ErrorCode, crate::state::*};

#[derive(Accounts)]
pub struct ClaimToken<'info> {
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
    #[account(
        init_if_needed,
        payer = authority,
        seeds = [ USER_ACCOUNT_SEED.as_bytes(), authority.key().as_ref() ],
        bump,
        space = USER_ACCOUNT_SIZE
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    //the authority allowed to transfer token
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub token_vault_bump: u8,
    pub presale_account_bump: u8,
}

#[access_control(is_finalized(&ctx.accounts.presale_account))]
pub fn handler(ctx: Context<ClaimToken>, nonce_vault: u8) -> Result<()> {
    let token_mint_key = ctx.accounts.token_mint.key();
    let seeds = &[
        token_mint_key.as_ref(), 
        &[nonce_vault], 
        &[ctx.accounts.token_vault_bump], 
        &[ctx.accounts.presale_account_bump]
    ];
    let signer = &[&seeds[..]];
    let amount =
        ctx.accounts.presale_account.presale_rate * ctx.accounts.user_account.user_buy_amount;

    if ctx.accounts.user_account.user_buy_amount <= 0 {
        return Err(error!(ErrorCode::NoClaimableToken));
    }

    if ctx.accounts.token_vault.amount < amount {
        return Err(error!(ErrorCode::PresaleNotEnoughToken));
    }
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
    ctx.accounts.user_account.is_claimed = 1;

    emit!(UserClaimed {
        user: *ctx.accounts.authority.key,
        amount: amount,
        time_stamp: Clock::get().unwrap().unix_timestamp as u32
    });
    Ok(())
}
