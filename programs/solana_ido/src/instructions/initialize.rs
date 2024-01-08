use anchor_spl::token::{Mint, Token, TokenAccount};
use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    // #[account(
    //     address = PRESALE_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    // )]
    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = admin,
        token::mint = token_mint,
        token::authority = token_vault, //the PDA address is both the vault account and the authority (and event the mint authority)
        seeds = [TOKEN_VAULT_SEED.as_bytes(), token_mint.key().as_ref() ],
        bump,
    )]
    ///the not-yet-created, derived token vault pubkey
    pub token_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [ ADMIN_MANAGE_SEED.as_bytes(), admin.key().as_ref() ],
        bump,
        space = ADMIN_ACCOUNT_SIZE
    )]
    pub admin_account: Box<Account<'info, AdminAccount>>,
    /// CHECK: This is vault account.
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [ SOL_VAULT_SEED.as_bytes() ],
        bump,
        space = 8 + 8
    )]
    pub escrow_account: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [ PRESALE_INFO_SEED.as_bytes() ],
        bump,
        space = PRESALE_ACCOUNT_SIZE
    )]
    pub presale_account: Box<Account<'info, PresaleAccount>>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.admin_account.super_admin = *ctx.accounts.admin.key;
    Ok(())
}
