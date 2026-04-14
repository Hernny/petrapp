// Importa el prelude de Anchor para `Pubkey`, macros y helpers
use anchor_lang::prelude::*;

#[account]
pub struct TicketingEvent {

    //PDA del evento
    pub id: Pubkey,

    // Quién controla el evento (organizador)
    pub organizer: Pubkey,

    // Moneda de cobro (ej: USDC mint)
    pub payment_mint: Pubkey,

    // Wallet/ATA recaudadora
    pub treasury: Pubkey,

    // Identidad legible on-chain
    // Máx. razonables para no reventar costos:
    // name: 64, description: 256, venue: 128, image_url: 256
    pub name: String,

    /// HASH SHA-256 (Keccak256 en Solana) de toda la metadata completa
    /// Este es el campo CRÍTICO para auditoría
    pub metadata_hash: [u8; 32],
    /// Enlace a la metadata enriquecida en tu DB
    /// (ej: IPFS hash o URL base)
    pub metadata_uri: String, 

    // Tiempos (timestamps en segundos)
    pub starts_at: i64,
    pub ends_at: i64,
    pub created_at: i64,
    pub updated_at: i64,

    // Configuración de tickets
    pub total_tickets: u64,
    pub sold_tickets: u64,
    pub max_tickets_per_wallet: u32,

    // Estado
    pub status: EventStatus,
    pub refund_policy: RefundPolicy,
    
    // Cancelación
    pub cancellation_reason: CancellationReason,
    pub cancelled_at: i64,
    
    // Control
    pub is_resellable: bool,
    pub metadata_version: u32, 

    pub bump: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum EventStatus {
    Upcoming,
    Live,
    Ended,
    Cancelled,
    Postponed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum RefundPolicy {
    NoRefund,
    FullRefund,
    FullRefundDaysAhead(u32),
    TieredRefund,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum CancellationReason {
    NotCancelled,
    VenueUnavailable,
    ArtistCancel,
    LowTicketSales,
    SecurityConcerns,
    NaturalDisaster,
    GovernmentRestriction,
    Other,
}

impl TicketingEvent {
    /// Tamaño total del PDA en bytes (729)
    pub const SPACE: usize = 8 +            // discriminator
        32 +                                // id
        32 +                                // organizer
        32 +                                // payment_mint
        32 +                                // treasury
        (4 + 64) +                          // name
        32 +                                // metadata_hash
        (4 + 512) +                         // metadata_uri
        8 + 8 + 8 + 8 +                     // timestamps
        8 + 8 + 4 +                         // tickets
        1 +                                 // status
        5 +                                 // refund_policy
        1 +                                 // cancellation_reason
        8 +                                 // cancelled_at
        1 +                                 // is_resellable
        4 +                                 // metadata_version
        1;                                  // bump
}
