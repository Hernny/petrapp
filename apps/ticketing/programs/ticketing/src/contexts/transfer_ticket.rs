use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::models::{TicketingEvent, Ticket};
use crate::error::TicketingError;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferTicketArgs {
    pub new_owner: Pubkey,
}

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    /// Owner actual del ticket que firma la tx
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Ticket PDA que debe estar a nombre del owner
    #[account(
        mut,
        constraint = ticket.owner == owner.key() @ TicketingError::NotTicketOwner,
        constraint = ticket.ticket_mint == ticket_mint.key(),
        constraint = ticket.event == event.key(),
    )]
    pub ticket: Account<'info, Ticket>,

    /// Evento asociado al ticket
    #[account()]
    pub event: Account<'info, TicketingEvent>,
     
    #[account(mut)]
    pub ticket_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = from_ata.mint == ticket_mint.key(),
        constraint = from_ata.owner == owner.key(),
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to_ata.mint == ticket_mint.key(),
        constraint = to_ata.owner == new_owner.key(),
    )]
    pub to_ata: Account<'info, TokenAccount>,

    /// CHECK: Validated safely via constraint on to_ata
    pub new_owner: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
