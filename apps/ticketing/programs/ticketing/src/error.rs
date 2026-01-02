use anchor_lang::prelude::*;

#[error_code]
pub enum TicketingError {
    #[msg("Event name is too long")]
    EventNameTooLong,

    #[msg("Event name is required")]
    EventNameRequired,

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
    EventInThePast,

    #[msg("Event is not active")]
    EventNotActive,

    #[msg("Event has been cancelled")]
    EventCancelled,

    #[msg("Event is sold out")]
    EventSoldOut,

    #[msg("Caller is not the ticket owner")]
    NotTicketOwner,

    #[msg("Ticket already used")]
    TicketAlreadyUsed,

    #[msg("Overflow in arithmetic")]
    Overflow,
}
