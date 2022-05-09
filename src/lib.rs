use anchor_lang::{AnchorSerialize, AnchorDeserialize};
use anchor_lang::prelude::*;

pub mod cpi_context_accounts;
pub mod cpi_util;
pub mod state;
pub mod stake_system;
pub mod validator_system;
pub mod liq_pool;
pub mod calc;
pub mod error;
pub mod located;
pub mod list;
pub mod checks;

/// The static program ID
pub static ID: Pubkey = Pubkey::new_from_array([
    5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
    155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
]); // "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"

//-----------------------------------------------------
#[derive(
    Clone, Copy, Debug, Default, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fee {
    pub basis_points: u32,
}