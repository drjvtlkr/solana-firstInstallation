use anchor_lang::prelude::*;

declare_id!("DJBJGfsuL3JZkypBwPmHMxrjuYKM85BvJWQLQLc97UHh");

#[program]
pub mod poi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
