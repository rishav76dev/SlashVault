use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod basic {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        budget: u64,
        start_time: i64,
        penalty_time: i64,
        terminate_time: i64,
    ) -> Result<()> {
        ctx.accounts.contract.init(
        ctx.accounts.company.key(),
        ctx.accounts.contractor.key(),
        budget,
        start_time,
        penalty_time,
        terminate_time,
        &ctx.bumps,
        
    )?;

    **ctx.accounts.company.to_account_info().try_borrow_mut_lamports()? -= budget;
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? += budget;

    Ok(())
    }
}


