use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("HWnSAW1x8PDdqKVEu7qBkvrJ7PvtTwn9NqLxTtsNiRvN");

#[program]
pub mod bus_trip_ticket {
    use super::*;

    pub fn initialize_bus_line(
        ctx: Context<InitializeAdminAccount>,
        to_three_km: u64,
        to_six_km: u64,
        to_twelve_km: u64,
        to_twenty_seven_km: u64,
        more_twenty_seven_km: u64
    ) -> Result<()> {
        let (_bus_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.signer.key().as_ref()], &Pubkey::from_str("HWnSAW1x8PDdqKVEu7qBkvrJ7PvtTwn9NqLxTtsNiRvN").unwrap());
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
    pub fn enable_card(
        ctx: Context<EnableCard>
    ) -> Result<()> {
        let (_user_card_pda, _bump): (Pubkey, u8) = Pubkey::find_program_address(&[b"Enable", ctx.accounts.signer.key().as_ref()], &Pubkey::from_str("HWnSAW1x8PDdqKVEu7qBkvrJ7PvtTwn9NqLxTtsNiRvN").unwrap());
        Ok(())
    }
    pub fn take_a_trip(
        ctx: Context<Trip>,
        _km: u8,
    ) -> Result<()> {
        let km_list = [ctx.accounts.bus.to_three_km,ctx.accounts.bus.to_six_km,ctx.accounts.bus.to_twelve_km,ctx.accounts.bus.to_twenty_seven_km,ctx.accounts.bus.more_twenty_seven_km].to_vec();
        let km: usize = _km as usize;
        require!(km < 6, ErrorCode::InvalidaKilometer);
        **ctx.accounts.card.to_account_info().try_borrow_mut_lamports()? -= km_list[km];
        **ctx.accounts.to.try_borrow_mut_lamports()? += km_list[km];
        Ok(())
    }
    pub fn charge_balance(
        ctx: Context<ChargeBalance>,
        amount: u64
    ) -> Result<()> {
        let transfer = system_instruction::transfer(
            &ctx.accounts.from.key(), &ctx.accounts.to.key(), amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[ctx.accounts.from.to_account_info(), ctx.accounts.to.to_account_info().clone()],
        ).expect("Error");
        msg!("Transfered Lamports");
        Ok(())
    }
    pub fn change_prices(
        ctx: Context<ChangePrices>,
        to_three_km: u64,
        to_six_km: u64,
        to_twelve_km: u64,
        to_twenty_seven_km: u64,
        more_twenty_seven_km: u64
    ) -> Result<()> {
        require_keys_eq!(ctx.accounts.signer.key(), ctx.accounts.bus.authority.key());
        let bus: &mut Account<BusAccount> = &mut ctx.accounts.bus;
        bus.to_three_km = to_three_km;
        bus.to_six_km = to_six_km;
        bus.to_twelve_km = to_twelve_km;
        bus.to_twenty_seven_km = to_twenty_seven_km;
        bus.more_twenty_seven_km = more_twenty_seven_km;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdminAccount<'info> {
    #[account(init, seeds = [signer.key().as_ref()], bump, payer = signer, space = 85)]
    pub bus: Account<'info, BusAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct EnableCard<'info> {
    #[account(init, seeds = [b"Enable", signer.key().as_ref()], bump, payer = signer, space = 8)]
    pub card: Account<'info, EnableUserCard>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Trip<'info> {
    #[account(mut, seeds = [bus.authority.key().as_ref()], bump = bus.bump_original)]
    pub bus: Account<'info, BusAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, seeds = [b"Enable", card.authority.key().as_ref()], bump = card.bump)]
    pub card: Account<'info, EnableUserCard>,
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ChargeBalance<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ChangePrices<'info> {
    #[account(mut, seeds = [bus.authority.key().as_ref()], bump = bus.bump_original)]
    pub bus: Account<'info, BusAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[account]
pub struct BusAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub to_three_km: u64,
    pub to_six_km: u64,
    pub to_twelve_km: u64,
    pub to_twenty_seven_km: u64,
    pub more_twenty_seven_km: u64
}
#[account]
pub struct EnableUserCard {
    pub authority: Pubkey, 
    pub bump_original: u8,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]InvalidaKilometer,
}