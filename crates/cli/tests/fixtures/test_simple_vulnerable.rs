use anchor_lang::prelude::*;

#[program]
pub mod test_program {
    use super::*;

    pub fn unsafe_lamport(ctx: Context<TestCtx>) -> Result<()> {
        // VULNERABILITY: Direct lamport manipulation
        **ctx.accounts.user.lamports.borrow_mut() -= 1000000;
        Ok(())
    }

    pub fn unsafe_arithmetic(ctx: Context<TestCtx>) -> Result<()> {
        // VULNERABILITY: Arithmetic overflow
        let mut counter: u32 = 0;
        counter = counter.saturating_add(u32::MAX);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TestCtx<'info> {
    #[account(mut)]
    pub user: AccountInfo<'info>,
}
