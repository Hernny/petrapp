use anchor_lang::prelude::*;
use crate::models::{TicketingEvent, RefundPolicy};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateEventArgs {
    pub name: String,                      // Max 64 bytes
    pub metadata_uri: String,              // Max 512 bytes (IPFS URL o JSON)
    pub metadata_hash: [u8; 32],           // SHA-256 hash
    pub starts_at: i64,                    // UNIX timestamp
    pub ends_at: i64,                      // UNIX timestamp
    pub total_tickets: u64,
    pub max_tickets_per_wallet: u32,
    pub is_resellable: bool,
    pub refund_policy: RefundPolicy,
}

#[derive(Accounts)]
#[instruction(args: CreateEventArgs)]
pub struct CreateEvent<'info> {
    /// Organizer que firma y paga la creación del evento
    #[account(mut)]
    pub organizer: Signer<'info>,

    /// Cuenta Event PDA
    #[account(
        init,
        payer = organizer,
        space = TicketingEvent::SPACE,
        // PDA por organizer
        seeds = [
            b"event", 
            organizer.key().as_ref(), 
            args.metadata_hash.as_ref()
        ],
        bump
    )]
    pub event: Account<'info, TicketingEvent>,

    pub system_program: Program<'info, System>,
}
