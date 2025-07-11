use anchor_lang::prelude::*;

declare_id!("3CncsNFNsevg2p8saW4fMjXBDoGXjzWjm1srG1CbmEj3");

#[program]
pub mod stake_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
