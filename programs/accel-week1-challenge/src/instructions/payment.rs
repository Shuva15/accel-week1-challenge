use anchor_lang::{prelude::*, system_program::{ Transfer, transfer }};

use crate::VaultState;
use crate::error::VaultError;

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let total_vault_lamports = self.vault.lamports();
        let rent_exempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        if amount > total_vault_lamports {
            return Err(error!(VaultError::InsufficientVaultBalance));
        } else if amount > total_vault_lamports - rent_exempt && amount <= total_vault_lamports {
            return Err(error!(VaultError::BelowRentExemption));
        }

        let cpi_accounts = Transfer {
        from: self.vault.to_account_info(),
        to: self.user.to_account_info()
        };

        let seeds = &[b"vault", self.vault_state.to_account_info().key.as_ref(), &[self.vault_state.vault_bump],];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}