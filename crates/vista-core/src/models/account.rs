use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data: Vec<u8>,
}