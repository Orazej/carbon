use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd5e005d16245775c")]
pub struct VirtualPool {
    pub volatility_tracker: VolatilityTracker,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub legacy_creation_fee_bits: u8,
    pub creation_fee_bits: u8,
    pub padding_0: [u8; 6],
    pub padding_1: [u64; 6],
}

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd5e005d16245775c")]
pub(crate) struct VirtualPoolLegacy {
    pub volatility_tracker: VolatilityTracker,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub padding_1: [u64; 7],
}

impl From<VirtualPoolLegacy> for VirtualPool {
    fn from(legacy: VirtualPoolLegacy) -> Self {
        Self {
            volatility_tracker: legacy.volatility_tracker,
            config: legacy.config,
            creator: legacy.creator,
            base_mint: legacy.base_mint,
            base_vault: legacy.base_vault,
            quote_vault: legacy.quote_vault,
            base_reserve: legacy.base_reserve,
            quote_reserve: legacy.quote_reserve,
            protocol_base_fee: legacy.protocol_base_fee,
            protocol_quote_fee: legacy.protocol_quote_fee,
            partner_base_fee: legacy.partner_base_fee,
            partner_quote_fee: legacy.partner_quote_fee,
            sqrt_price: legacy.sqrt_price,
            activation_point: legacy.activation_point,
            pool_type: legacy.pool_type,
            is_migrated: legacy.is_migrated,
            is_partner_withdraw_surplus: legacy.is_partner_withdraw_surplus,
            is_protocol_withdraw_surplus: legacy.is_protocol_withdraw_surplus,
            migration_progress: legacy.migration_progress,
            is_withdraw_leftover: legacy.is_withdraw_leftover,
            is_creator_withdraw_surplus: legacy.is_creator_withdraw_surplus,
            migration_fee_withdraw_status: legacy.migration_fee_withdraw_status,
            metrics: legacy.metrics,
            finish_curve_timestamp: legacy.finish_curve_timestamp,
            creator_base_fee: legacy.creator_base_fee,
            creator_quote_fee: legacy.creator_quote_fee,
            legacy_creation_fee_bits: 0,
            creation_fee_bits: 0,
            padding_0: [0; 6],
            padding_1: [0; 6],
        }
    }
}
