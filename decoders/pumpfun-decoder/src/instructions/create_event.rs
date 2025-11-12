use carbon_core::{
    borsh::{self, BorshDeserialize},
    deserialize::{extract_discriminator, CarbonDeserialize},
};
use std::io::{ErrorKind, Read};

#[derive(
    Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub timestamp: i64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub token_total_supply: u64,
    pub token_program: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

const CREATE_EVENT_DISCRIMINATOR: [u8; 16] =
    [228, 69, 165, 46, 81, 203, 154, 29, 27, 114, 169, 77, 222, 235, 99, 118];
const LEGACY_TOKEN_PROGRAM: solana_pubkey::Pubkey = solana_pubkey::Pubkey::new_from_array([
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133,
    237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
]);

impl borsh::BorshDeserialize for CreateEvent {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            name: String::deserialize_reader(reader)?,
            symbol: String::deserialize_reader(reader)?,
            uri: String::deserialize_reader(reader)?,
            mint: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            bonding_curve: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            user: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            creator: solana_pubkey::Pubkey::deserialize_reader(reader)?,
            timestamp: i64::deserialize_reader(reader)?,
            virtual_token_reserves: u64::deserialize_reader(reader)?,
            virtual_sol_reserves: u64::deserialize_reader(reader)?,
            real_token_reserves: u64::deserialize_reader(reader)?,
            token_total_supply: u64::deserialize_reader(reader)?,
            token_program: read_pubkey_with_default(reader, LEGACY_TOKEN_PROGRAM)?,
            is_mayhem_mode: read_bool_with_default(reader, false)?,
        })
    }
}

impl CarbonDeserialize for CreateEvent {
    const DISCRIMINATOR: &'static [u8] = &CREATE_EVENT_DISCRIMINATOR;

    fn deserialize(data: &[u8]) -> Option<Self> {
        let (disc, payload) = extract_discriminator(Self::DISCRIMINATOR.len(), data)?;
        if disc != Self::DISCRIMINATOR {
            return None;
        }
        let mut cursor = payload;
        Self::deserialize_reader(&mut cursor).ok()
    }
}

fn read_pubkey_with_default<R: Read>(
    reader: &mut R,
    default: solana_pubkey::Pubkey,
) -> std::io::Result<solana_pubkey::Pubkey> {
    match solana_pubkey::Pubkey::deserialize_reader(reader) {
        Ok(value) => Ok(value),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(default),
        Err(err) => Err(err),
    }
}

fn read_bool_with_default<R: Read>(reader: &mut R, default: bool) -> std::io::Result<bool> {
    match bool::deserialize_reader(reader) {
        Ok(value) => Ok(value),
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => Ok(default),
        Err(err) => Err(err),
    }
}
