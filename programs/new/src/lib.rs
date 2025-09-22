use anchor_lang::prelude::*;

declare_id!("G1Utk3esKdSrdP7tKzzzz71kpfS1KHV9RgrwCkgyjbk6");

#[program]
pub mod new {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
