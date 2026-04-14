use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::models::{TicketingEvent, Ticket};
use crate::error::TicketingError;

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
    pub event: Account<'info, TicketingEvent>,

    #[account(mut)]
    pub ticket_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = buyer_ticket_ata.mint == ticket_mint.key() @ TicketingError::InvalidTokenMint,
        constraint = buyer_ticket_ata.owner == buyer.key() @ TicketingError::InvalidTokenOwner,
    )]
    pub buyer_ticket_ata: Account<'info, TokenAccount>,

    #[account(address = event.payment_mint)]
    pub payment_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = buyer_payment_ata.mint == event.payment_mint,
        constraint = buyer_payment_ata.owner == buyer.key(),
    )]
    pub buyer_payment_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_payment_ata.mint == event.payment_mint,
        constraint = treasury_payment_ata.owner == event.treasury,
    )]
    pub treasury_payment_ata: Account<'info, TokenAccount>,

    /// CHECK: Metaplex Program validates this
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Metaplex Program validates this
    #[account(mut)]
    pub master_edition_account: UncheckedAccount<'info>,

    /// CHECK: Sysvar de instrucciones
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub sysvar_instructions: UncheckedAccount<'info>,

    /// CHECK: Metaplex Token Metadata Program ID
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

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
