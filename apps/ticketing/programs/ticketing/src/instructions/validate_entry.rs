use anchor_lang::prelude::*;
use crate::contexts::ValidateEntry;
use crate::error::TicketingError;

pub fn handler(ctx: Context<ValidateEntry>) -> Result<()> {
    let ticket = &mut ctx.accounts.ticket;

    // No permitir reuso
    require!(!ticket.used, TicketingError::TicketAlreadyUsed);

    let now = Clock::get()?.unix_timestamp;

    ticket.used = true;
    ticket.used_at = now;
    ticket.updated_at = now;

    Ok(())
}
