use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}, };

use crate::{state::AmmAccount, InitializeBumps};

#[derive(Accounts)]
pub struct InitilizeAmm<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = AmmAccount::INIT_SPACE,
        seeds = [b"amm"],
        bump
    )]
    pub amm: Account<'info, AmmAccount>,
    
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(
        init,
        payer = payer,
        mint::decimal = 6,
        mint::authority = config.key(),
        seeds = [b"lp", amm.key().as_ref()],
        bump,
    )]
    pub mint_lp: Account<'info, Mint>,
    
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = amm,
        associated_token::program = token_program
    )]
    pub token_account_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = payer,
        associated_token::mint= mint_b,
        associated_token::authority = amm,
        associated_token::program = token_program
    )]
    pub token_account_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> InitilizeAmm<'info>{
    pub fn initialize_amm(&mut self, fee: u64, authority: Option<Pubkey>, bump: &InitilizeAmmBumps, seeds: u64) -> Result<()> {
        self.amm.set_inner(AmmAccount { 
            seed: seeds, 
            authority: authority, 
            token_a: self.mint_a.key(), 
            token_b: self.mint_b.key(), 
            fee: fee, 
            locked: false, 
            amm_bump: bump.amm, 
            lp_bump: bump.mint_lp
        });
        Ok(())
    }
}