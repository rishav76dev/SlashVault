use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {

  #[account(mut)]
  pub company: Signer<'info>,

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
    seeds = [b"vault", contract.key().as_ref()],
    bump,
    space = 8
  )]
  pub vault: SystemAccount<'info>,

  pub system_program: Program<'info, System>,

}
impl Contract {
    pub fn init(
        &mut self,
        company: Pubkey,
        contractor: Pubkey,
        budget: u64,
        start_time: i64,
        penalty_time: i64,
        terminate_time: i64,
        bumps: &InitializeBumps,
        
    ) -> Result<()> {
        self.bump = bumps.contract;
        self.company = company;
        self.contractor = contractor;
        self.budget = budget;
        self.start_time = start_time;
        self.penalty_time = penalty_time;
        self.terminate_time = terminate_time;

        // transfer budget into vault


        Ok(())
    }
}
