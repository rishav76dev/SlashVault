use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

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
        // All logic is placed directly in this function, accessing `ctx.accounts` and `ctx.bumps`.
        let contract = &mut ctx.accounts.contract;
        let company_key = ctx.accounts.company.key();
        let contractor_key = ctx.accounts.contractor.key();

        // Use the automatically generated bumps struct from the Context
        let contract_bump = ctx.bumps.contract;

        // Set the contract data directly.
        contract.set_inner(Contract {
            bump: contract_bump,
            company: company_key,
            contractor: contractor_key,
            budget,
            start_time,
            penalty_time,
            terminate_time,
        });

        // CPI to transfer funds, including the company signer.
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.company.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        // Transfer funds. No PDA signing is needed for this transfer as the company is the one paying.
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, budget)?;

        Ok(())
    }
}
