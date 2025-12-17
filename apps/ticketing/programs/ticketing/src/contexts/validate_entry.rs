use anchor_lang::prelude::*;
use crate::models::Ticket;

#[derive(Accounts)]
pub struct ValidateEntry<'info> {
    /// Persona / sistema que valida la entrada (organizador o staff)
    pub validator: Signer<'info>,

    /// Ticket a validar
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
}
