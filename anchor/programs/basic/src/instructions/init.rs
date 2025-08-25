use anchor_lang::prelude::*;
use crate::Contract;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub company: Signer<'info>,

    /// CHECK: The account is verified by its use as a seed below.
    pub contractor: UncheckedAccount<'info>,

    #[account(
        init,
        payer = company,
        space = Contract::INIT_SPACE,
        seeds = [b"contract", company.key().as_ref(), contractor.key().as_ref()],
        bump
    )]
    pub contract: Account<'info, Contract>,

    #[account(
    init,
    payer = company,
    space = 0,
    seeds = [b"vault", contract.key().as_ref()],
    bump,
    )]
    /// CHECK: This PDA is only used to hold lamports. No data is read or written.
    pub vault: AccountInfo<'info>,

    // Right now your vault is just a bare lamport holder. That’s fine if you only need to escrow/deposit SOL.
    // But if later you want to:
    // Track balances,
    // Or store some metadata,
    // then you’d need to give it a custom Account type instead of just lamports.

    pub system_program: Program<'info, System>,
}
