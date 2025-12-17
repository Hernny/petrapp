// Importa el prelude de Anchor para que `Pubkey`, macros y traits
// como `AccountSerialize`/`AccountDeserialize` estén disponibles.
use anchor_lang::prelude::*;

// Deriva Clone para cumplir los bounds de `Account<T>` en Anchor 0.32
#[account]
pub struct Ticket {
    // ID/folio interno
    pub external_id: String,

    pub owner: Pubkey,
    pub event: Pubkey,
    // Wallet que refirió al comprador (opcional)
    pub referrer: Pubkey, // puede ser Pubkey::default() si no hubo referral

    // Datos de control
    pub purchased_at: i64,
    pub created_at: i64,
    pub updated_at: i64,

    // Estado de uso
    pub used: bool,
    pub used_at: i64,
    pub transfer_count: u32,

    // Info de ubicación dentro del evento
    pub seat_number: String, // ej: "A-12", "Fila B - 05"
    pub room_name: String,   // ej: "Sala Principal", "Stage 2"

    // Tipo / categoría (opcional pero útil)
    pub category: String,    // VIP, GENERAL, etc.

    pub price_amount: u64,   // sigue sirviendo para crypto/futuro
}
