use anchor_lang::prelude::*;

declare_id!("4Whhcd4H1ud4RgeJV7uczjyWmTRiBHt1ioKiu9bEFYAX");

#[program]
pub mod zk_verification {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
