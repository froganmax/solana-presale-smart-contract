use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
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
pub fn handler(ctx: Context<RemoveWhitelist>, addresses: Vec<String>) -> Result<()> {
    for old_user in addresses.iter() {
        match ctx
            .accounts
            .presale_account
            .white_list
            .iter()
            .position(|og| og == old_user)
        {
            Some(index) => {
                ctx.accounts.presale_account.white_list.remove(index);
            }
            None => {}
        }
    }
    Ok(())
}
