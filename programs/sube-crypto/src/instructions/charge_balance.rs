use anchor_lang::{
    prelude::*,
    solana_program::system_instruction
}; 

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

#[derive(Accounts)]
pub struct ChargeBalance<'info> {
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}