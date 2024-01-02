use {
    crate::{errors::ErrorCode},
    anchor_lang::prelude::*,
};

pub const SOL_VAULT_SEED: &str = "presale-escrow-vault";
pub const ADMIN_MANAGE_SEED: &str = "admin-role";
pub const USER_ACCOUNT_SEED: &str = "user-role";
pub const PRESALE_INFO_SEED: &str = "presale-info";
pub const PRESALE_TOKEN_MINT_PUBKEY: &str = "85hr9mrrv2SHuWsEB58y7HhyKM76C88gVo8W5ee4fvcu";

pub const ADMIN_ACCOUNT_SIZE: usize = 8 + std::mem::size_of::<AdminAccount>() + 8;
pub const USER_ACCOUNT_SIZE: usize = 8 + std::mem::size_of::<UserAccount>() + 8;
pub const PRESALE_ACCOUNT_SIZE: usize = 8 + std::mem::size_of::<PresaleAccount>() + 32 * 100 + 8;

#[account]
pub struct AdminAccount {
    pub super_admin: Pubkey,
}

#[account]
pub struct UserAccount {
    pub user_buy_amount: u64,
    pub is_claimed: u8
}

#[account]
pub struct PresaleAccount {
    pub team_percent: u32,  // 10% : 1000
    pub presale_rate: u64,
    pub has_whitelist: u8,
    pub user_max_buy: u64,
    pub start_time: u32,
    pub end_time: u32,
    pub white_list: Vec<String>,
    pub total_participants: u32,
    pub total_sol_amount: u64,
    pub total_token_amount: u64,
    pub is_finalized: u8,
    pub is_cancelled: u8
}

#[event]
pub struct UserBought {
    pub user: Pubkey,
    pub amount: u64,
    pub time_stamp: u32
}

#[event]
pub struct UserClaimed {
    pub user: Pubkey,
    pub amount: u64,
    pub time_stamp: u32
}

#[event]
pub struct RefundToken {
    pub user: Pubkey,
    pub amount: u64,
    pub time_stamp: u32
}

#[event]
pub struct UpdatePresalePeriod {
    pub start_time: u32,
    pub end_time: u32,
    pub time_stamp: u32
}

#[event]
pub struct PresaleFinalized {
    pub total_amount: u64,
    pub time_stamp: u32
}

#[event]
pub struct PresaleCancelled {
    pub total_amount: u64,
    pub time_stamp: u32
}

pub fn is_admin<'info>(
    admin_account: &Account<'info, AdminAccount>,
    signer: &Signer<'info>,
) -> Result<()> {
    if admin_account.super_admin != *signer.key {
        return Err(error!(ErrorCode::AccessDenied));
    }
    Ok(())
}

pub fn is_finalized<'info>(
    presale_account: &Account<'info, PresaleAccount>
) -> Result<()> {
    if presale_account.is_finalized != 0 {
        return Err(error!(ErrorCode::PresaleNotFinished))
    }
    Ok(())
}

pub fn is_cancelled<'info>(
    presale_account: &Account<'info, PresaleAccount>
) -> Result<()> {
    if presale_account.is_cancelled != 1 {
        return Err(error!(ErrorCode::PresaleNotCancelled))
    }
    Ok(())
}

pub fn is_presale_live<'info>(
    presale_account: &Account<'info, PresaleAccount>
) -> Result<()> {
    if presale_account.start_time > Clock::get().unwrap().unix_timestamp as u32 {
        return Err(error!(ErrorCode::PresaleNotStarted))
    }
    if presale_account.end_time < Clock::get().unwrap().unix_timestamp as u32 {
        return Err(error!(ErrorCode::PresaleAlreadyEnded))
    }
    Ok(())
}