use anchor_lang::prelude::*;

#[program]
mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
                      authority: Pubkey) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.authority = authority;
        counter.count = 0;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub counter: ProgramAccount<'info, Counter>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: ProgramAccount<'info, Counter>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
}
