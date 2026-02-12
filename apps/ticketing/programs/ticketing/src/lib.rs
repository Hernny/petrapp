use anchor_lang::prelude::*;

pub mod models;
pub mod contexts;
pub mod instructions;
pub mod error;

use contexts::*; 

declare_id!("Da8QHfrUBh2duJuwQ4PTB7GupnJknxjRTZV1b8wkfcj");

#[program]
pub mod ticketing {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
        instructions::create_event::handler(ctx, args)
    }

    pub fn mint_ticket(ctx: Context<MintTicket>, args: MintTicketArgs) -> Result<()> {
        instructions::mint_ticket::handler(ctx, args)
    }

    pub fn validate_entry(ctx: Context<ValidateEntry>) -> Result<()> {
        instructions::validate_entry::handler(ctx)
    }

    pub fn transfer_ticket(ctx: Context<TransferTicket>, args: TransferTicketArgs) -> Result<()> {
        instructions::transfer_ticket::handler(ctx, args)
    }

}

