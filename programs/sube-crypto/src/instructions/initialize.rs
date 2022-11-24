use anchor_lang::prelude::*;
use crate::state::accounts::*;

pub fn initialize_bus_line(
    ctx: Context<InitializeAdminAccount>,
    to_three_km: u64,
    to_six_km: u64,
    to_twelve_km: u64,
    to_twenty_seven_km: u64,
    more_twenty_seven_km: u64
) -> Result<()> {
    let (_bus_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.signer.key().as_ref()], 
    ctx.program_id);
    let bus: &mut Account<BusAccount> = &mut ctx.accounts.bus;
    bus.authority = ctx.accounts.signer.key();
    bus.bump_original = bump;
    bus.to_three_km = to_three_km;
    bus.to_six_km = to_six_km;
    bus.to_twelve_km = to_twelve_km;
    bus.to_twenty_seven_km = to_twenty_seven_km;
    bus.more_twenty_seven_km = more_twenty_seven_km;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAdminAccount<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 85)]
    pub bus: Account<'info, BusAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}