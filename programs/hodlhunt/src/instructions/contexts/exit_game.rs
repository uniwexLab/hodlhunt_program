use crate::state::{Fish, Ocean};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExitGame<'info> {
    #[account(mut)]
    pub ocean: Account<'info, Ocean>,

    #[account(
        mut,
        constraint = fish.owner == owner.key()
    )]
    pub fish: Account<'info, Fish>,

    #[account(
        mut,
        seeds = [b"vault", ocean.key().as_ref()],
        bump = ocean.vault_bump
    )]
    /// CHECK: PDA vault
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = admin.key() == ocean.admin
    )]
    /// CHECK: Admin must match ocean.admin
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Fish name registry PDA; verified by derivation in handler
    pub name_registry: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
