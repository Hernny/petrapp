use anchor_lang::prelude::*;
use crate::models::Event;
use crate::constants::{
    EVENT_SPACE,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateEventArgs {
    pub name: String,
    pub description: String,
    pub venue: String,
    pub image_url: String,
    pub external_id: String,
    pub starts_at: i64,
    pub ends_at: i64,
    pub total_tickets: u64,
    pub max_tickets_per_wallet: u32,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    /// Organizer que firma y paga la creación del evento
    #[account(mut)]
    pub organizer: Signer<'info>,

    /// Cuenta Event PDA
    #[account(
        init,
        payer = organizer,
        space = EVENT_SPACE,
        // PDA por organizer
        seeds = [b"event", organizer.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,

    pub system_program: Program<'info, System>,
}
