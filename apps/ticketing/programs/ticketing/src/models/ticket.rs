// Importa el prelude de Anchor para que `Pubkey`, macros y traits
// como `AccountSerialize`/`AccountDeserialize` estén disponibles.
use anchor_lang::prelude::*;

// Deriva Clone para cumplir los bounds de `Account<T>` en Anchor 0.32
#[account]
pub struct Ticket { 
    // Identidad
    pub ticket_mint: Pubkey,            // ✅ SPL Mint
    pub event: Pubkey,                  // ✅ PDA del evento

    pub original_buyer: Pubkey,
    pub owner: Pubkey, 
    pub referrer: Pubkey, // puede ser Pubkey::default() si no hubo referral
    // Wallet que refirió al comprador (opcional)

    /// HASH SHA-256 (Keccak256 en Solana) de toda la metadata completa
    /// Este es el campo CRÍTICO para auditoría
    pub metadata_hash: [u8; 32],
    
    /// Enlace a la metadata enriquecida en tu DB
    /// (ej: IPFS hash o URL base)
    pub metadata_uri: String, 

    // Datos de control 
    pub created_at: i64,
    pub updated_at: i64, 
    pub last_transfer_at: i64,
    pub used_at: i64,

    // Estado de uso
    pub status: TicketStatus,
    pub transfer_count: u32,  

    pub price_amount: u64,

    pub bump: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum TicketStatus {
    Active,
    Used,
    Refunded,
    Cancelled,
}

impl Ticket {
    /// Tamaño total del PDA en bytes (762)
    pub const SPACE: usize = 8 +            // discriminator
        32 +                                // ticket_mint
        32 +                                // event
        32 +                                // original_buyer
        32 +                                // owner
        32 +                                // referrer
        32 +                                // metadata_hash
        (4 + 512) +                         // metadata_uri
        8 + 8 + 8 + 8 +                     // timestamps
        1 +                                 // status
        4 +                                 // transfer_count
        8 +                                 // price_amount
        1;                                  // bump
}
