use anchor_lang::prelude::*;
use crate::contexts::{TransferTicket, TransferTicketArgs};
use crate::error::TicketingError;
use crate::models::{Ticket, TicketingEvent,TicketStatus, EventStatus};

pub fn handler(ctx: Context<TransferTicket>, args: TransferTicketArgs) -> Result<()> {
    let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;
    let event: &Account<TicketingEvent> = &ctx.accounts.event;
    // ✅ Validar que el evento está activo (no cancelado, no terminado)
    require!(
        event.status != EventStatus::Cancelled,
        TicketingError::EventCancelled
    );

    // ✅ Validar que el evento no ha terminado
    let now = Clock::get()?.unix_timestamp;
    require!(
        now <= event.ends_at,
        TicketingError::EventHasFinished
    );

    // ✅ Validar que el ticket está activo (no ha sido usado)
    require!(
        ticket.status == TicketStatus::Active,
        TicketingError::TicketAlreadyUsed
    );

    let current_owner = ticket.owner;

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
    ticket.last_transfer_at = now;

    emit!(TicketTransferred {
        ticket: ticket.key(),
        from: current_owner,
        to: args.new_owner,
        transfer_count: ticket.transfer_count,
        transferred_at: now,
    });

    Ok(())
}
#[event]
pub struct TicketTransferred {
    pub ticket: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub transfer_count: u32,
    pub transferred_at: i64,
}