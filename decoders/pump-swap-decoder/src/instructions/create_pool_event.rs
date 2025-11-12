use carbon_core::{
    borsh::{self, BorshDeserialize},
    deserialize::{extract_discriminator, CarbonDeserialize},
};
use std::io::{ErrorKind, Read};

#[derive(
    Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub coin_creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

const CREATE_POOL_EVENT_DISCRIMINATOR: [u8; 16] =
    [228, 69, 165, 46, 81, 203, 154, 29, 177, 49, 12, 210, 160, 118, 167, 116];

impl borsh::BorshDeserialize for CreatePoolEvent {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: i64::deserialize_reader(reader)?,
            index: u16::deserialize_reader(reader)?,
            creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            base_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            quote_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            base_mint_decimals: u8::deserialize_reader(reader)?,
            quote_mint_decimals: u8::deserialize_reader(reader)?,
            base_amount_in: u64::deserialize_reader(reader)?,
            quote_amount_in: u64::deserialize_reader(reader)?,
            pool_base_amount: u64::deserialize_reader(reader)?,
            pool_quote_amount: u64::deserialize_reader(reader)?,
            minimum_liquidity: u64::deserialize_reader(reader)?,
            initial_liquidity: u64::deserialize_reader(reader)?,
            lp_token_amount_out: u64::deserialize_reader(reader)?,
            pool_bump: u8::deserialize_reader(reader)?,
            pool: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            lp_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            user_base_token_account: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            user_quote_token_account: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            coin_creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            is_mayhem_mode: read_bool_with_default(reader, false)?,
        })
    }
}

impl CarbonDeserialize for CreatePoolEvent {
    const DISCRIMINATOR: &'static [u8] = &CREATE_POOL_EVENT_DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        let (disc, payload) = extract_discriminator(Self::DISCRIMINATOR.len(), data)?;
        if disc != Self::DISCRIMINATOR {
            return None;
        }
        let mut cursor = payload;
        Self::deserialize_reader(&mut cursor).ok()
    }
}

fn read_bool_with_default<R: Read>(reader: &mut R, default: bool) -> std::io::Result<bool> {
    match bool::deserialize_reader(reader) {
        Ok(value) => Ok(value),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(default),
        Err(err) => Err(err),
    }
}
