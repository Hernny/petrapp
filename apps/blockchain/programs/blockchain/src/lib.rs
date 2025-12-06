use anchor_lang::prelude::*;

declare_id!("Fus7LSp5DX6sh6zQ3Yvcs7kBYbfmocY2G3LTBVk7yeZJ");

#[program]
pub mod blockchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
