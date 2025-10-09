use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapParameters2 {
    /// When it's exact in, partial fill, this will be amount_in. When it's exact out, this will be amount_out
    pub amount_0: u64,
    /// When it's exact in, partial fill, this will be minimum_amount_out. When it's exact out, this will be maximum_amount_in
    pub amount_1: u64,
    /// Swap mode, refer [SwapMode]
    pub swap_mode: u8,
}
