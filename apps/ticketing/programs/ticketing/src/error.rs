use anchor_lang::prelude::*;

#[error_code]
pub enum TicketingError {
    // ===== Event Creation Errors =====
    #[msg("Event name is too long")]
    EventNameTooLong,

    #[msg("Event name is required")]
    EventNameRequired,

    #[msg("Event external ID is too long")]
    EventExternalIdTooLong,

    #[msg("Metadata URI is too long")]
    MetadataUriTooLong,

    #[msg("Metadata URI is required")]
    MetadataUriRequired,

    #[msg("Total tickets must be greater than zero")]
    TotalTicketsMustBePositive,

    #[msg("Max tickets per wallet must be greater than zero")]
    MaxPerWalletMustBePositive,

    #[msg("Invalid event time range")]
    InvalidEventTimeRange,

    #[msg("Event starts in the past")]
    EventStartInThePast,

    // ===== Ticket Minting Errors =====
    #[msg("Event is not active")]
    EventNotActive,

    #[msg("Event has been cancelled")]
    EventCancelled,

    #[msg("Event is sold out")]
    EventSoldOut,

    #[msg("Event has finished")]
    EventHasFinished,

    #[msg("Invalid ticket price")]
    InvalidTicketPrice,

    #[msg("Invalid token mint")]
    InvalidTokenMint,

    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    // ===== Ticket Operations Errors =====
    #[msg("Caller is not the ticket owner")]
    NotTicketOwner,

    #[msg("Ticket already used")]
    TicketAlreadyUsed,

    #[msg("Ticket is not active")]
    TicketNotActive,

    #[msg("Invalid transfer recipient")]
    InvalidTransferRecipient,

    #[msg("Cannot transfer to self")]
    CannotTransferToSelf,

    // ===== Arithmetic and System Errors =====
    #[msg("Overflow in arithmetic")]
    Overflow,

    #[msg("Underflow in arithmetic")]
    Underflow,

    // ===== Metaplex Integration Errors =====
    #[msg("Error creating Metaplex metadata account")]
    MetadataCreationFailed,

    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,

    #[msg("Invalid metadata hash")]
    InvalidMetadataHash,

    // ===== Generic Errors =====
    #[msg("Invalid argument")]
    InvalidArgument,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Account not found")]
    AccountNotFound,
}