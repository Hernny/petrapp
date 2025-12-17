use anchor_lang::prelude::*;
use crate::contexts::{CreateEvent, CreateEventArgs};
use crate::constants::{
    EVENT_NAME_MAX_LEN,
    EVENT_EXTERNAL_ID_MAX_LEN,
};
use crate::error::TicketingError;
pub fn handler(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
    // Validar longitudes de strings
    require!(
        args.name.len() <= EVENT_NAME_MAX_LEN,
        TicketingError::EventNameTooLong
    );
    require!(
        args.external_id.len() <= EVENT_EXTERNAL_ID_MAX_LEN,
        TicketingError::EventExternalIdTooLong
    );

    let organizer = &ctx.accounts.organizer;
    let event = &mut ctx.accounts.event;

    let now = Clock::get()?.unix_timestamp;

    // Opcional: validaciones de negocio básicas
    require!(args.total_tickets > 0, TicketingError::TotalTicketsMustBePositive);
    require!(args.max_tickets_per_wallet > 0, TicketingError::MaxPerWalletMustBePositive);
    require!(args.starts_at < args.ends_at, TicketingError::InvalidEventTimeRange);

    // Rellenar cuenta Event
    event.organizer = organizer.key();

    event.name = args.name;
    event.external_id = args.external_id;

    event.starts_at = args.starts_at;
    event.ends_at = args.ends_at;
    event.created_at = now;
    event.updated_at = now;

    event.total_tickets = args.total_tickets;
    event.sold_tickets = 0;
    event.max_tickets_per_wallet = args.max_tickets_per_wallet;

    event.is_active = true;
    event.is_cancelled = false;

    Ok(())
}
