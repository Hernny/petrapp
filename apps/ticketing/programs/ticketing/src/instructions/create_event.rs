use anchor_lang::prelude::*;

use crate::contexts::{CreateEvent, CreateEventArgs};
use crate::models::{EventStatus, CancellationReason};
use crate::error::TicketingError;

pub fn handler(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
    // ✅ Validaciones de nombre
    require!(args.name.len() > 0, TicketingError::EventNameRequired);
    require!(args.name.len() <= 64, TicketingError::EventNameTooLong);

    // ✅ Validaciones de metadata_uri
    require!(args.metadata_uri.len() > 0, TicketingError::MetadataUriRequired);
    require!(args.metadata_uri.len() <= 512, TicketingError::MetadataUriTooLong);

    // ✅ Números básicos
    require!(
        args.total_tickets > 0,
        TicketingError::TotalTicketsMustBePositive
    );
    require!(
        args.max_tickets_per_wallet > 0,
        TicketingError::MaxPerWalletMustBePositive
    );

    // ✅ Tiempos
    require!(
        args.starts_at < args.ends_at,
        TicketingError::InvalidEventTimeRange
    );

    let now = Clock::get()?.unix_timestamp;

    require!(
        args.starts_at >= now,
        TicketingError::EventInThePast
    );

    // ✅ Rellenar Event (cuenta ya creada por el context con seeds + metadata_hash)
    let event = &mut ctx.accounts.event;

    //Guardar ID (PDA del evento)
    event.id = event.key();

    // Guardar bump para auditoría
    event.bump = ctx.bumps["event"];

    event.organizer = ctx.accounts.organizer.key();
    event.name = args.name;
    event.metadata_uri = args.metadata_uri;
    event.metadata_hash = args.metadata_hash;

    event.starts_at = args.starts_at;
    event.ends_at = args.ends_at;

    event.created_at = now;
    event.updated_at = now;

    event.total_tickets = args.total_tickets;
    event.sold_tickets = 0;
    event.max_tickets_per_wallet = args.max_tickets_per_wallet;

    event.status = EventStatus::Upcoming;
    event.refund_policy = args.refund_policy;
    event.is_resellable = args.is_resellable;

    event.cancellation_reason = CancellationReason::NotCancelled;
    event.cancelled_at = 0;
    event.metadata_version = 1;

    // ✅ Evento para auditoría on-chain
    emit!(EventCreated {
        event: event.key(),
        organizer: ctx.accounts.organizer.key(),
        metadata_hash: args.metadata_hash,
        created_at: now,
    });

    Ok(())
}

#[event]
pub struct EventCreated {
    pub event: Pubkey,
    pub organizer: Pubkey,
    pub metadata_hash: [u8; 32],
    pub created_at: i64,
}
