use anchor_lang::prelude::*;

declare_id!("FRWd8BMsaUWXSK5Ma25oFKtGd5WCvvJhBDWpUqqbCcd6");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
