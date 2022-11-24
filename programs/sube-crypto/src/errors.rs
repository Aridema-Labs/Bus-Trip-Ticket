use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]InvalidaKilometer,
    #[msg("Insuficient SOL")]InsuficientSOL, 
    #[msg("You are not the authority")]AuthorityError,
}