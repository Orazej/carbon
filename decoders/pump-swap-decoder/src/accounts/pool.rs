use carbon_core::{
    borsh::{self, BorshDeserialize},
    deserialize::{extract_discriminator, CarbonDeserialize},
};
use std::io::{ErrorKind, Read};

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub lp_supply: u64,
    pub coin_creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

const POOL_DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

impl borsh::BorshDeserialize for Pool {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            pool_bump: u8::deserialize_reader(reader)?,
            index: u16::deserialize_reader(reader)?,
            creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            base_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            quote_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            lp_mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            pool_base_token_account: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            pool_quote_token_account: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            lp_supply: u64::deserialize_reader(reader)?,
            coin_creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            is_mayhem_mode: read_bool_with_default(reader, false)?,
        })
    }
}

impl CarbonDeserialize for Pool {
    const DISCRIMINATOR: &'static [u8] = &POOL_DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        let (_, payload) = extract_discriminator(Self::DISCRIMINATOR.len(), data)?;
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
