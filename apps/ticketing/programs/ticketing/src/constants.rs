// No necesitas prelude aquí
// use anchor_lang::prelude::*;

// -------- Event --------

pub const EVENT_NAME_MAX_LEN: usize = 64;
pub const EVENT_DESC_MAX_LEN: usize = 256;
pub const EVENT_VENUE_MAX_LEN: usize = 128;
pub const EVENT_IMAGE_URL_MAX_LEN: usize = 256;
pub const EVENT_EXTERNAL_ID_MAX_LEN: usize = 64;

pub const EVENT_SPACE: usize = 8  // discriminant
    + 32                          // organizer: Pubkey
    + 4 + EVENT_NAME_MAX_LEN      // name
    + 4 + EVENT_DESC_MAX_LEN      // description
    + 4 + EVENT_VENUE_MAX_LEN     // venue
    + 4 + EVENT_IMAGE_URL_MAX_LEN // image_url
    + 4 + EVENT_EXTERNAL_ID_MAX_LEN // external_id
    + 8 * 4                       // starts_at, ends_at, created_at, updated_at
    + 8                           // total_tickets
    + 8                           // sold_tickets
    + 4                           // max_tickets_per_wallet
    + 1                           // is_active
    + 1;                          // is_cancelled

// -------- Ticket --------

pub const TICKET_EXTERNAL_ID_MAX_LEN: usize = 64;
pub const TICKET_CATEGORY_MAX_LEN: usize = 32;
// Longitudes máximas para ubicación del asiento / sala
pub const TICKET_SEAT_MAX_LEN: usize = 32;
pub const TICKET_ROOM_MAX_LEN: usize = 64;

pub const TICKET_SPACE: usize = 8  // discriminant
    + 4 + TICKET_EXTERNAL_ID_MAX_LEN // external_id: String
    + 32                           // owner: Pubkey
    + 32                           // event: Pubkey
    + 32                           // referrer: Pubkey
    + 8 * 3                        // purchased_at, created_at, updated_at
    + 1                            // used
    + 8                            // used_at
    + 4                            // transfer_count
    + 4 + TICKET_CATEGORY_MAX_LEN  // category: String
    + 4 + TICKET_SEAT_MAX_LEN      // seat_number: String
    + 4 + TICKET_ROOM_MAX_LEN      // room_name: String
    + 8;                           // price_amount
