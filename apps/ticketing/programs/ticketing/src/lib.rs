use anchor_lang::prelude::*;

declare_id!("3zFJJ44BbmXKaTAg31Jgf1TDWRENTYU6E11R691HkHad");

#[program]
pub mod ticketing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
