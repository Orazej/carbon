use carbon_core::{
    borsh::{self, BorshDeserialize},
    deserialize::{extract_discriminator, CarbonDeserialize},
};
use std::io::{ErrorKind, Read};

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct Global {
    pub initialized: bool,
    pub authority: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [solana_pubkey::Pubkey; 7],
    pub set_creator_authority: solana_pubkey::Pubkey,
    pub admin_set_creator_authority: solana_pubkey::Pubkey,
    pub create_v2_enabled: bool,
    pub whitelist_pda: solana_pubkey::Pubkey,
    pub reserved_fee_recipient: solana_pubkey::Pubkey,
    pub mayhem_mode_enabled: bool,
}

const GLOBAL_DISCRIMINATOR: [u8; 8] = [167, 232, 232, 177, 200, 108, 114, 127];

impl borsh::BorshDeserialize for Global {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let initialized = bool::deserialize_reader(reader)?;
        let authority = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        let fee_recipient = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        let initial_virtual_token_reserves = u64::deserialize_reader(reader)?;
        let initial_virtual_sol_reserves = u64::deserialize_reader(reader)?;
        let initial_real_token_reserves = u64::deserialize_reader(reader)?;
        let token_total_supply = u64::deserialize_reader(reader)?;
        let fee_basis_points = u64::deserialize_reader(reader)?;
        let withdraw_authority = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        let enable_migrate = bool::deserialize_reader(reader)?;
        let pool_migration_fee = u64::deserialize_reader(reader)?;
        let creator_fee_basis_points = u64::deserialize_reader(reader)?;
        let mut fee_recipients = [solana_pubkey::Pubkey::default(); 7];
        for recipient in fee_recipients.iter_mut() {
            *recipient = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        }
        let set_creator_authority = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        let admin_set_creator_authority = solana_pubkey::Pubkey::deserialize_reader(reader)?;
        let create_v2_enabled = read_bool_with_default(reader, false)?;
        let whitelist_pda =
            read_pubkey_with_default(reader, solana_pubkey::Pubkey::default())?;
        let reserved_fee_recipient =
            read_pubkey_with_default(reader, solana_pubkey::Pubkey::default())?;
        let mayhem_mode_enabled = read_bool_with_default(reader, false)?;

        Ok(Self {
            initialized,
            authority,
            fee_recipient,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
            withdraw_authority,
            enable_migrate,
            pool_migration_fee,
            creator_fee_basis_points,
            fee_recipients,
            set_creator_authority,
            admin_set_creator_authority,
            create_v2_enabled,
            whitelist_pda,
            reserved_fee_recipient,
            mayhem_mode_enabled,
        })
    }
}

impl CarbonDeserialize for Global {
    const DISCRIMINATOR: &'static [u8] = &GLOBAL_DISCRIMINATOR;

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
