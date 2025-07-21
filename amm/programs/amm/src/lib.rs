use anchor_lang::prelude::*;
pub mod error;
pub mod constraint;
pub mod state;
pub mod instructions;

declare_id!("9CfuGeuZiNMEXm8D3bwpjUwUnuaK69he2cMPZQcTwYZe");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
