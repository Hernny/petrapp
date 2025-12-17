use anchor_lang::prelude::*;
use crate::models::{Event, Ticket};
use crate::constants::{
    TICKET_SPACE,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTicketArgs {
    pub external_id: String,
    pub category: String,
    pub price_amount: u64,
}

#[derive(Accounts)]
pub struct MintTicket<'info> {
    /// Comprador del ticket (puede ser wallet custodia de PetrApp en v1)
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// Evento al que pertenece el ticket
    #[account(mut)]
    pub event: Account<'info, Event>,

    /// Cuenta Ticket PDA
    #[account(
        init,
        payer = buyer,
        space = TICKET_SPACE,
        seeds = [b"ticket", event.key().as_ref(), buyer.key().as_ref()],
        bump
    )]

    pub ticket: Account<'info, Ticket>,

    pub system_program: Program<'info, System>,
}
