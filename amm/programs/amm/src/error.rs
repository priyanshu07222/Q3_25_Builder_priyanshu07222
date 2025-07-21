use anchor_lang::{error_code, prelude};


#[error_code]
pub enum AmmError{
    #[msg("Invalid Amount")]
    InvalidAmount
}