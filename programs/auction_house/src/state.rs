use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[account]
pub struct AuctionHouse {
    pub auction_house_fee_account: Pubkey,
    pub auction_house_treasury: Pubkey,
    pub treasury_withdrawal_destination: Pubkey,
    pub fee_withdrawal_destination: Pubkey,
    pub treasury_mint: Pubkey,
    pub authority: Pubkey,
    pub creator: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    pub fee_payer_bump: u8,
    pub seller_fee_basis_points: u16,
    pub can_change_sale_price: bool,
    pub escrow_payment_bump: u8,
    pub has_auctioneer: bool,
    pub auctioneer_address: Pubkey,
}

#[account]
pub struct Auctioneer {
    pub auctioneer_authority: Pubkey,
    pub auction_house: Pubkey,
    pub bump: u8,
}
