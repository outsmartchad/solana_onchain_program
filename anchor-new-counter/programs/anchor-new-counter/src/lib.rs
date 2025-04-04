use anchor_lang::prelude::*;

declare_id!("2MU2A7wNQpssSLkegNWg4NstsVX92p3P91UP22NUrjDi");

#[program]
pub mod anchor_new_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created!");
        msg!("Current Count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        
        // Check for overflow before incrementing
        if counter.count == u64::MAX {
            return err!(CounterError::CounterOverflow);
        }
        
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter count incremented");
        msg!("Current Count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        
        // Check if counter is at 0
        if counter.count == 0 {
            return err!(CounterError::CounterAtZero);
        }
        
        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter count decremented");
        msg!("Current Count: {}", counter.count);
        Ok(())
    }
}

#[error_code]
pub enum CounterError {
    #[msg("Counter cannot be decremented below zero")]
    CounterAtZero,
    #[msg("Counter has reached maximum value")]
    CounterOverflow,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

const DISCRIMINATOR: usize = 8;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}