use anchor_lang::prelude::*;
use crate::state::accounts::*;

pub fn enable_card(
    ctx: Context<EnableCard>
) -> Result<()> {
    let (_user_card_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[b"Enable", ctx.accounts.signer.key().as_ref()], ctx.program_id);
    let card: &mut Account<EnableUserCard> = &mut ctx.accounts.card;
    card.authority = ctx.accounts.signer.key();
    card.bump_original = bump;
    Ok(())
}

#[derive(Accounts)]
pub struct EnableCard<'info> {
    #[account(init, seeds = [b"Enable", signer.key().as_ref()], bump, payer = signer, space = 41)]
    pub card: Account<'info, EnableUserCard>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}