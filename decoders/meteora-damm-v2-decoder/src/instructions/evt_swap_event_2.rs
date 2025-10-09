use carbon_core::{borsh, CarbonDeserialize};

use crate::types::{SwapParameters2, SwapResult2};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbd4233a826507599")]
pub struct EvtSwapEvent2 {
    pub pool: solana_pubkey::Pubkey,
    pub trade_direction: u8,
    pub collect_fee_mode: u8,
    pub has_referral: bool,
    pub params: SwapParameters2,
    // excluded_transfer_fee_amount_in is swap_result.included_fee_amount_in
    pub swap_result: SwapResult2,
    pub included_transfer_fee_amount_in: u64,
    pub included_transfer_fee_amount_out: u64,
    pub excluded_transfer_fee_amount_out: u64,
    pub current_timestamp: u64,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
}
