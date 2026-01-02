use anchor_lang::prelude::*;
use crate::contexts::{MintTicket, MintTicketArgs};
use crate::models::{EventStatus, TicketStatus};
use crate::error::TicketingError;

pub fn handler(ctx: Context<MintTicket>, args: MintTicketArgs) -> Result<()> {
    // ===== VALIDACIONES =====

    // Metadata
    require!(
        args.metadata_uri.len() > 0,
        TicketingError::MetadataUriRequired
    );
    require!(
        args.metadata_uri.len() <= 512,
        TicketingError::MetadataUriTooLong
    );

    // Precio
    require!(
        args.price_amount > 0,
        TicketingError::InvalidTicketPrice
    );

    let buyer = &ctx.accounts.buyer;
    let event = &mut ctx.accounts.event;
    let ticket = &mut ctx.accounts.ticket;
    let now = Clock::get()?.unix_timestamp;

    // Estado del evento
    match event.status {
        EventStatus::Upcoming | EventStatus::Live => {},
        _ => return Err(TicketingError::EventNotActive.into()),
    }

    require!(
        event.status != EventStatus::Cancelled,
        TicketingError::EventCancelled
    );

    // Stock
    require!(
        event.sold_tickets < event.total_tickets,
        TicketingError::EventSoldOut
    );

    // Timing
    require!(
        now <= event.ends_at,
        TicketingError::EventHasFinished
    );

    // ===== RELLENAR TICKET =====

    ticket.ticket_mint = ctx.accounts.ticket_mint.key();
    ticket.event = event.key();
    ticket.original_buyer = buyer.key();
    ticket.owner = buyer.key();
    ticket.referrer = args.referrer;

    ticket.metadata_uri = args.metadata_uri;
    ticket.metadata_hash = args.metadata_hash;

    ticket.created_at = now;
    ticket.updated_at = now;
    ticket.last_transfer_at = now;
    ticket.used_at = 0;

    ticket.status = TicketStatus::Active;
    ticket.transfer_count = 0;
    ticket.price_amount = args.price_amount;

    // ===== ACTUALIZAR EVENTO =====

    event.sold_tickets = event
        .sold_tickets
        .checked_add(1)
        .ok_or(TicketingError::Overflow)?;
    event.updated_at = now;

    // ===== EMITIR EVENTOS =====

    emit!(TicketMinted {
        ticket: ticket.key(),
        event: event.key(),
        ticket_mint: ctx.accounts.ticket_mint.key(),
        buyer: buyer.key(),
        original_buyer: ticket.original_buyer,
        referrer: args.referrer,
        metadata_hash: args.metadata_hash,
        price_amount: args.price_amount,
        total_sold: event.sold_tickets,
        minted_at: now,
    });

    if event.sold_tickets == event.total_tickets {
        emit!(EventSoldOut {
            event: event.key(),
            total_tickets: event.total_tickets,
            sold_out_at: now,
        });
    }

    Ok(())
}

#[event]
pub struct TicketMinted {
    pub ticket: Pubkey,
    pub event: Pubkey,
    pub ticket_mint: Pubkey,
    pub buyer: Pubkey,
    pub original_buyer: Pubkey,
    pub referrer: Pubkey,
    pub metadata_hash: [u8; 32],
    pub price_amount: u64,
    pub total_sold: u64,
    pub minted_at: i64,
}

#[event]
pub struct EventSoldOut {
    pub event: Pubkey,
    pub total_tickets: u64,
    pub sold_out_at: i64,
}
