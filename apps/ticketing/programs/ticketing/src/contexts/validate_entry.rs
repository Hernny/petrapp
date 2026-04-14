use anchor_lang::prelude::*;
use crate::models::{TicketingEvent, Ticket};
use crate::error::TicketingError;

#[derive(Accounts)]
pub struct ValidateEntry<'info> {
    /// Persona / sistema que valida la entrada (organizador o staff)
    pub validator: Signer<'info>,

    /// Ticket a validar
    #[account(
        mut,
        constraint = ticket.event == event.key() @ TicketingError::InvalidArgument
    )]
    pub ticket: Account<'info, Ticket>,

    /// Evento asociado al ticket
    #[account(
        constraint = event.organizer == validator.key() @ TicketingError::Unauthorized
    )]
    pub event: Account<'info, TicketingEvent>,
     
    pub system_program: Program<'info, System>,
}
