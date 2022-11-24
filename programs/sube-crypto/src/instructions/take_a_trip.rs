use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
};
use crate::errors::ErrorCode; 
use crate::state::accounts::*;

pub fn take_a_trip(
    ctx: Context<Trip>,
    _km: u8,
) -> Result<()> {
    let km_list = [ctx.accounts.bus.to_three_km,ctx.accounts.bus.to_six_km,ctx.accounts.bus.to_twelve_km,ctx.accounts.bus.to_twenty_seven_km,ctx.accounts.bus.more_twenty_seven_km].to_vec();
    let km: usize = _km as usize;
    require!(km < 6, ErrorCode::InvalidaKilometer);
    require!(AccountInfo::lamports(&ctx.accounts.card.to_account_info()) > (km_list[km] + 5000), ErrorCode::InsuficientSOL);
    **ctx.accounts.card.to_account_info().try_borrow_mut_lamports()? -= km_list[km];
    **ctx.accounts.to.try_borrow_mut_lamports()? += km_list[km];
    **ctx.accounts.card.to_account_info().try_borrow_mut_lamports()? -= 5000;
    **ctx.accounts.from.try_borrow_mut_lamports()? += 5000;
    Ok(())
}
#[derive(Accounts)]
pub struct Trip<'info> {
    #[account(mut, seeds = [bus.authority.key().as_ref()], bump = bus.bump_original)]
    pub bus: Account<'info, BusAccount>,
    #[account(mut, seeds = [b"Enable", card.authority.key().as_ref()], bump = card.bump_original)]
    pub card: Account<'info, EnableUserCard>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}