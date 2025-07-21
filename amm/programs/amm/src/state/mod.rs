use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmmAccount{
    /// Unique identifier used to differentiate between multiple AMM pools.
    /// Helps in deriving unique PDAs for each AMM instance.
    pub seed: u64,
    
    /// Optional authority/maintainer of the AMM.
    /// Can be used to control parameters like fees or to pause the AMM.
    /// If set to `None`, the AMM is fully decentralized.
    pub authority: Option<Pubkey>,
    
    /// SPL token mint address for Token A (first asset in the pair).
    pub token_a: Pubkey,
    
    /// SPL token mint address for Token B (second asset in the pair).
    pub token_b: Pubkey,
    
    /// Swap fee taken on each trade (in basis points, e.g., 30 = 0.3%).
    /// This fee typically goes to liquidity providers or protocol treasury.
    pub fee: u64,
    
    /// Boolean flag to lock the AMM.
    /// When `true`, operations like swap or deposit can be disabled.
    pub locked: bool,
    
    /// Bump used to derive the PDA for this config account.
    /// Ensures the correct address is derived on-chain.
    pub config_bump: u8,
    
    /// Bump used to derive the PDA for the LP token mint account.
    /// LP tokens represent a user’s share of the liquidity pool.
    pub lp_bump: u8
}