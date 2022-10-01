use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("Cmyo3foHLuN7X48qiUBXdXzwz6hMjMebcQ2v3YzUKVp8");

#[program]
pub mod bus_trip_ticket {
    use super::*;

    pub fn initialize_bus_line(
        ctx: Context<InitializeAdminAccount>,
        to3km: u64,
        to6km: u64,
        to12km: u64,
        to27km: u64,
        more27km: u64
    ) -> Result<()> {
        let (_bus_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.signer.key().as_ref()], &Pubkey::from_str("Cmyo3foHLuN7X48qiUBXdXzwz6hMjMebcQ2v3YzUKVp8").unwrap());
        let bus: &mut Account<BusAccount> = &mut ctx.accounts.bus;
        bus.authority = ctx.accounts.signer.key();
        bus.bump_original = bump;
        bus.to3km = to3km;
        bus.to6km = to6km;
        bus.to12km = to12km;
        bus.to27km = to27km;
        bus.more27km = more27km;
        Ok(())
    }
    pub fn take_a_trip(
        ctx: Context<Trip>,
        _km: u8,
    ) -> Result<()> {
        let km: usize = _km as usize;
        require!(km < 6, ErrorCode::InvalidaKilometer);
        let km_list = [ctx.accounts.bus.to3km,ctx.accounts.bus.to6km,ctx.accounts.bus.to12km,ctx.accounts.bus.to27km,ctx.accounts.bus.more27km].to_vec();
        let transfer = system_instruction::transfer(
            &ctx.accounts.from.key(), &ctx.accounts.to.key(), km_list[km],
        );
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        if km == 0 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 1 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 5 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 6 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 7 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
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
pub struct Trip<'info> {
    #[account(mut, seeds = [bus.authority.key().as_ref()], bump = bus.bump_original)]
    pub bus: Account<'info, BusAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct BusAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub to3km: u64,
    pub to6km: u64,
    pub to12km: u64,
    pub to27km: u64,
    pub more27km: u64
}
#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]InvalidaKilometer
}