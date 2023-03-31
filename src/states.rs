use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub role: String,
    pub product_count: u8,
    pub last_product_idx: u8,
}

#[account]
#[derive(Default)]
pub struct ProductAccount {
    pub authority: Pubkey, 
    pub idx: u8,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub price: String,
    pub quantity: u64,
}

#[account]
#[derive(Default)]
pub struct TransactionAccount {
    pub authority: Pubkey, //32
    pub product_authority: Pubkey, //32
    pub product_idx: u8,           // 1
    pub transaction_type: String,     // 4 + 256
    pub transaction_hash: String, // 4 + 2048
    pub amount: String,     // 4 + 256
    pub created_at: String,  //  4 + 256
}