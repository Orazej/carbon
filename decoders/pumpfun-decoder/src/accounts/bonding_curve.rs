use carbon_core::{
    borsh::{self, BorshDeserialize},
    deserialize::{extract_discriminator, CarbonDeserialize},
};
use std::io::{ErrorKind, Read};

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

const BONDING_CURVE_DISCRIMINATOR: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];

impl borsh::BorshDeserialize for BondingCurve {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            virtual_token_reserves: u64::deserialize_reader(reader)?,
            virtual_sol_reserves: u64::deserialize_reader(reader)?,
            real_token_reserves: u64::deserialize_reader(reader)?,
            real_sol_reserves: u64::deserialize_reader(reader)?,
            token_total_supply: u64::deserialize_reader(reader)?,
            complete: bool::deserialize_reader(reader)?,
            creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            is_mayhem_mode: read_bool_with_default(reader, false)?,
        })
    }
}

impl CarbonDeserialize for BondingCurve {
    const DISCRIMINATOR: &'static [u8] = &BONDING_CURVE_DISCRIMINATOR;

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
