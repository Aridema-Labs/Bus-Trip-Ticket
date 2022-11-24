use anchor_lang::prelude::*;

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