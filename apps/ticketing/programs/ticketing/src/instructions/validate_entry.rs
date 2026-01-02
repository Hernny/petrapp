use anchor_lang::prelude::*;
use crate::contexts::ValidateEntry;
use crate::error::TicketingError;
use crate::models::TicketStatus;

pub fn handler(ctx: Context<ValidateEntry>) -> Result<()> {
    let ticket = &mut ctx.accounts.ticket;

    // No permitir reuso
    require!(ticket.status == TicketStatus::Active, TicketingError::TicketAlreadyUsed);

    let now = Clock::get()?.unix_timestamp;

    ticket.status = TicketStatus::Used;
    ticket.used_at = now;
    ticket.updated_at = now;

    Ok(())
}
