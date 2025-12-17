use anchor_lang::prelude::*;
use crate::contexts::{MintTicket, MintTicketArgs};
use crate::constants::{
    TICKET_EXTERNAL_ID_MAX_LEN,
    TICKET_CATEGORY_MAX_LEN,
}; 
use crate::error::TicketingError;

pub fn handler(ctx: Context<MintTicket>, args: MintTicketArgs) -> Result<()> {
    // Validar longitudes
    require!(
        args.external_id.len() <= TICKET_EXTERNAL_ID_MAX_LEN,
        TicketingError::TicketExternalIdTooLong
    );
    require!(
        args.category.len() <= TICKET_CATEGORY_MAX_LEN,
        TicketingError::CategoryTooLong
    );

    let buyer = &ctx.accounts.buyer;
    let event = &mut ctx.accounts.event;
    let ticket = &mut ctx.accounts.ticket;

    let now = Clock::get()?.unix_timestamp;

    // Regla 1: evento activo y no cancelado
    require!(event.is_active, TicketingError::EventNotActive);
    require!(!event.is_cancelled, TicketingError::EventCancelled);

    // Regla 2: no vendido más allá del total
    require!(
        event.sold_tickets < event.total_tickets,
        TicketingError::EventSoldOut
    );

    // TODO (futuro): max_tickets_per_wallet
    // Aquí, para cumplirlo bien, necesitarías contar tickets existentes
    // del mismo (event, buyer) off-chain o con otra PDA agregada.

    // Rellenar ticket
    ticket.external_id = args.external_id;
    ticket.owner = buyer.key();
    ticket.event = event.key();

    ticket.purchased_at = now;
    ticket.created_at = now;
    ticket.updated_at = now;

    ticket.used = false;
    ticket.used_at = 0;
    ticket.transfer_count = 0;

    ticket.category = args.category;
    ticket.price_amount = args.price_amount;

    // Actualizar evento
    event.sold_tickets = event
        .sold_tickets
        .checked_add(1)
        .ok_or(TicketingError::Overflow)?;
    event.updated_at = now;

    Ok(())
}