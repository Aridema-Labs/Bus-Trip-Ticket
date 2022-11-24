use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("CqtPfRjEWtqRR1XZq4EkfSUimCPxPiie7UcrWFJ2DxVV");

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
        instructions::initialize::initialize_bus_line(
            ctx, 
            to_three_km,
            to_six_km,
            to_twelve_km,
            to_twenty_seven_km,
            more_twenty_seven_km
        )
    }
    pub fn enable_card(
        ctx: Context<EnableCard>
    ) -> Result<()> {
        instructions::enable_card::enable_card(ctx)
    }
    pub fn take_a_trip(
        ctx: Context<Trip>,
        _km: u8,
    ) -> Result<()> {
        instructions::take_a_trip::take_a_trip(
            ctx, 
            _km
        )
    }
    pub fn charge_balance(
        ctx: Context<ChargeBalance>,
        amount: u64
    ) -> Result<()> {
        instructions::charge_balance::charge_balance(
            ctx,
            amount
        )
    }
    pub fn change_prices(
        ctx: Context<ChangePrices>,
        to_three_km: u64,
        to_six_km: u64,
        to_twelve_km: u64,
        to_twenty_seven_km: u64,
        more_twenty_seven_km: u64
    ) -> Result<()> {
        instructions::change_prices::change_prices(
            ctx,
            to_three_km,
            to_six_km,
            to_twelve_km,
            to_twenty_seven_km,
            more_twenty_seven_km
        )
    }
}