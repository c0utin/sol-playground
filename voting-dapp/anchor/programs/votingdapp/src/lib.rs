#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod votingdapp {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        id: u64,
        description: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.id = id;
        poll.description = description;
        poll.start = start;
        poll.end = end;
        poll.candidate_amount = 0;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
       init,
       payer = signer,
       space = Poll::INIT_SPACE + ANCHOR_DISCRIMINATOR_SIZE,
       seeds = [id.to_le_bytes().as_ref()],
       bump,
   )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub id: u64,
    #[max_len(280)]
    pub description: String,
    pub start: u64,
    pub end: u64,
    pub candidate_amount: u64,
}
