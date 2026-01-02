use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::models::{Event, Ticket};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTicketArgs {
    pub metadata_uri: String,      // Max 512 bytes
    pub metadata_hash: [u8; 32],   // SHA-256 hash
    pub price_amount: u64,
    pub referrer: Pubkey,
}

#[derive(Accounts)]
pub struct MintTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub event: Account<'info, Event>,

    pub ticket_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = buyer_ticket_ata.mint == ticket_mint.key() @ TicketingError::InvalidTokenMint,
        constraint = buyer_ticket_ata.owner == buyer.key() @ TicketingError::InvalidTokenOwner,
    )]
    pub buyer_ticket_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = buyer,
        space = Ticket::SPACE,
        seeds = [b"ticket", ticket_mint.key().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
