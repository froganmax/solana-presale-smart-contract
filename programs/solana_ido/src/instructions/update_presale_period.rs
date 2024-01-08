use {crate::account, crate::errors::ErrorCode, crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct UpdatePresalePeriod<'info> {
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
pub fn handler(
    ctx: Context<UpdatePresalePeriod>,
    price: u64,
    start_time: u64,
    end_time: u64,
) -> Result<()> {
    let cur_time = Clock::get().unwrap().unix_timestamp as u64;
    if cur_time > end_time || start_time >= end_time {
        return Err(error!(ErrorCode::WrongTimePeriod));
    }
    ctx.accounts.presale_account.price = price;
    ctx.accounts.presale_account.start_time = start_time;
    ctx.accounts.presale_account.end_time = end_time;
    Ok(())
}
