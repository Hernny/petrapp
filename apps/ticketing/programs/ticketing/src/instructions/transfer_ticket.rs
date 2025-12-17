use anchor_lang::prelude::*;
use crate::contexts::{TransferTicket, TransferTicketArgs};
use crate::models::Ticket;
use crate::error::TicketingError;

pub fn handler(ctx: Context<TransferTicket>, args: TransferTicketArgs) -> Result<()> {
    let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;

    // No permitir transferir tickets usados
    require!(!ticket.used, TicketingError::TicketAlreadyUsed);

    // ownership ya se validó en el contexto (constraint del Accounts)
    ticket.owner = args.new_owner;

    // Actualizar contador de transferencias
    ticket.transfer_count = ticket
        .transfer_count
        .checked_add(1)
        .ok_or(TicketingError::Overflow)?;

    // Actualizar timestamp de última modificación
    let now = Clock::get()?.unix_timestamp;
    ticket.updated_at = now;

    Ok(())
}
