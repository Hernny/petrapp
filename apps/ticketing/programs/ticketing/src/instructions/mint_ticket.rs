use anchor_lang::prelude::*;
use crate::contexts::{MintTicket, MintTicketArgs};
use crate::models::{EventStatus, TicketStatus};
use crate::error::TicketingError;

use anchor_spl::token::{self, MintTo, TransferChecked};
use mpl_token_metadata::types::TokenStandard;

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

    // ===== COBRO DEL TICKET (USDC) =====
    {
        let transfer_cpi_accounts = TransferChecked {
            from: ctx.accounts.buyer_payment_ata.to_account_info(),
            mint: ctx.accounts.payment_mint.to_account_info(),
            to: ctx.accounts.treasury_payment_ata.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let transfer_cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_cpi_accounts,
        );
        token::transfer_checked(
            transfer_cpi_ctx,
            args.price_amount,
            ctx.accounts.payment_mint.decimals,
        )?;
    }

    // ===== MINTEO DEL NFT (SPL Token) =====
    // El PDA del evento firma la creación y acuñación del NFT del ticket
    let event_key = event.organizer;
    let metadata_hash = event.metadata_hash;
    let bump = event.bump;
    
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"event",
        event_key.as_ref(),
        metadata_hash.as_ref(),
        &[bump],
    ]];

    {
        let mint_cpi_accounts = MintTo {
            mint: ctx.accounts.ticket_mint.to_account_info(),
            to: ctx.accounts.buyer_ticket_ata.to_account_info(),
            authority: event.to_account_info(),
        };
        let mint_cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_cpi_accounts,
            signer_seeds,
        );
        token::mint_to(mint_cpi_ctx, 1)?; // Supply = 1
    }

    // ===== METAPLEX MASTER EDITION =====
    {
        use mpl_token_metadata::instructions::CreateV1CpiBuilder;
        
        CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program.to_account_info())
            .metadata(&ctx.accounts.metadata_account.to_account_info())
            .master_edition(Some(&ctx.accounts.master_edition_account.to_account_info()))
            .mint(&ctx.accounts.ticket_mint.to_account_info(), true)
            .authority(&event.to_account_info())
            .payer(&ctx.accounts.buyer.to_account_info())
            .update_authority(&event.to_account_info(), true)
            .system_program(&ctx.accounts.system_program.to_account_info())
            .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
            .token_standard(TokenStandard::NonFungible)
            .name(format!("Ticket: {}", event.name).chars().take(32).collect())
            .uri(args.metadata_uri.clone())
            .seller_fee_basis_points(0)
            .invoke_signed(signer_seeds)?;
    }

    // ===== RELLENAR TICKET =====

    ticket.bump = ctx.bumps.ticket;
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
