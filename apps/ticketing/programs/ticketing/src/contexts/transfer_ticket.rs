use anchor_lang::prelude::*;
use crate::models::Ticket;
use crate::error::TicketingError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferTicketArgs {
    pub new_owner: Pubkey,
}

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    /// Current owner of the ticket
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Ticket PDA must estar actualmente a nombre de `owner`
    #[account(
        mut,
        constraint = ticket.owner == owner.key() @ TicketingError::NotTicketOwner
    )]
    pub ticket: Account<'info, Ticket>,
}
