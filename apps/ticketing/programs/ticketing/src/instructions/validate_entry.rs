use anchor_lang::prelude::*;
use crate::contexts::ValidateEntry;
use crate::error::TicketingError;
use crate::models::{Ticket, TicketingEvent,TicketStatus, EventStatus};

pub fn handler(ctx: Context<ValidateEntry>) -> Result<()> {
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
    // ✅ Marcar el ticket como usado
    ticket.status = TicketStatus::Used;
    ticket.used_at = now;
    ticket.updated_at = now;

    // ✅ Emitir evento para auditoría
    emit!(TicketValidated {
        ticket: ticket.key(),
        validated_at: now,
    });

    Ok(())
}
#[event]
pub struct TicketValidated {
    pub ticket: Pubkey,
    pub validated_at: i64,
}