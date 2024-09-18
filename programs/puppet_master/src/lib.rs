use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData; // Import the SetData accounts structure from the puppet program's CPI module
use puppet::program::Puppet; // Import the Puppet program definition for CPI (This is used to specify the program you want to invoke with CPI.)
use puppet::{self, Data}; // Import the puppet program and Data struct

declare_id!("B5T18uMD585s3zJyfpAASN6VDsjAAeyXipyyP36odR21");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        puppet::cpi::set_data(ctx.accounts.set_data_ctx(), data)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>, // client sends the public key
}

impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
