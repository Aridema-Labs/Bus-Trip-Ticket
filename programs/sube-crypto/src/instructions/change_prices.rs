use anchor_lang::prelude::*;
use crate::errors::ErrorCode; 
use crate::state::accounts::*;

pub fn change_prices(
    ctx: Context<ChangePrices>,
    to_three_km: u64,
    to_six_km: u64,
    to_twelve_km: u64,
    to_twenty_seven_km: u64,
    more_twenty_seven_km: u64
) -> Result<()> {
    require_keys_eq!(ctx.accounts.signer.key(), ctx.accounts.bus.authority.key(), ErrorCode::AuthorityError);
    let bus: &mut Account<BusAccount> = &mut ctx.accounts.bus;
    bus.to_three_km = to_three_km;
    bus.to_six_km = to_six_km;
    bus.to_twelve_km = to_twelve_km;
    bus.to_twenty_seven_km = to_twenty_seven_km;
    bus.more_twenty_seven_km = more_twenty_seven_km;
    Ok(())
}

#[derive(Accounts)]
pub struct ChangePrices<'info> {
    #[account(mut, seeds = [bus.authority.key().as_ref()], bump = bus.bump_original)]
    pub bus: Account<'info, BusAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}