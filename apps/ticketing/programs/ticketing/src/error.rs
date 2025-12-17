use anchor_lang::prelude::*;

#[error_code]
pub enum TicketingError {
    #[msg("Event name is too long")]
    EventNameTooLong,
    #[msg("Event external_id is too long")]
    EventExternalIdTooLong,
    #[msg("Total tickets must be greater than zero")]
    TotalTicketsMustBePositive,
    #[msg("Max tickets per wallet must be greater than zero")]
    MaxPerWalletMustBePositive,
    #[msg("Invalid event time range")]
    InvalidEventTimeRange,

    #[msg("Event is not active")]
    EventNotActive,
    #[msg("Event has been cancelled")]
    EventCancelled,
    #[msg("Event is sold out")]
    EventSoldOut,

    #[msg("Ticket external_id too long")]
    TicketExternalIdTooLong,
    #[msg("Category too long")]
    CategoryTooLong,

    #[msg("Caller is not the ticket owner")]
    NotTicketOwner,
    #[msg("Ticket already used")]
    TicketAlreadyUsed,
    #[msg("Overflow in arithmetic")]
    Overflow,
}
