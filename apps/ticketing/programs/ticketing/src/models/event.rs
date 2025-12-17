// Importa el prelude de Anchor para `Pubkey`, macros y helpers
use anchor_lang::prelude::*;

#[account]
pub struct Event {
    // Quién controla el evento (organizador)
    pub organizer: Pubkey,

    // Identidad legible on-chain
    // Máx. razonables para no reventar costos:
    // name: 64, description: 256, venue: 128, image_url: 256
    pub name: String,
    pub description: String,
    pub venue: String,
    pub image_url: String,  

   // Enlace a Web2 si quieres seguir reforzando el vínculo
    pub external_id: String,

    // Tiempos (timestamps en segundos)
    pub starts_at: i64,
    pub ends_at: i64,
    pub created_at: i64,
    pub updated_at: i64,

    // Configuración de tickets
    pub total_tickets: u64,
    pub sold_tickets: u64,
    pub max_tickets_per_wallet: u32,

    // Flags de estado
    pub is_active: bool,
    pub is_cancelled: bool,
}
