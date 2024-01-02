use {anchor_lang::prelude::*, crate::state::*};

#[derive(Accounts)]
pub struct AddWhitelist<'info> {
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
pub fn handler(ctx: Context<AddWhitelist>, addresses: Vec<String>) -> Result<()> {
    for new_user in addresses.iter() {
        if ctx
            .accounts
            .presale_account
            .white_list
            .iter()
            .find(|&og| og == new_user)
            == None
        {
            ctx.accounts
                .presale_account
                .white_list
                .push(new_user.to_string());
        }
    }
    Ok(())
}
