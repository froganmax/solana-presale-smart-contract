use anchor_lang::prelude::*;

declare_id!("73FRpgaZw6KbXvtS7qaDpXDvfY6S9S32ybqqSEHwgZ4W");

#[program]
pub mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, Anchor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
