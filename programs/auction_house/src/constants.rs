pub const PREFIX: &str = "auction_house";
pub const FEE_PAYER: &str = "fee_payer";
pub const TREASURY: &str = "treasury";
pub const SIGNER: &str = "signer";
pub const AUCTIONEER: &str = "auctioneer";
pub const TRADE_STATE_SIZE: usize = 1;
pub const AUCTIONEER_SIZE: usize = 8 +                      // Anchor discriminator/sighash
32 +                                                        // Auctioneer authority
32 +                                                        // Auction house instance
1 +                                                         // bump
63                                                          // Padding
;

pub const AUCTION_HOUSE_SIZE: usize = 8 +                   // key
32 +                                                        // fee Payer
32 +                                                        // treasury
32 +                                                        // treasury_withdrawal_destination
32 +                                                        // fee withdrawal destination
32 +                                                        // treasury mint
32 +                                                        // authority
32 +                                                        // creator
1 +                                                         // bump
1 +                                                         // treasury_bump
1 +                                                         // fee_payer_bump
2 +                                                         // seller fee basis points
1 +                                                         // can change sale price
8 +                                                         // escrow payment bump
1 +                                                         // has external auctioneer program as an authority
32 +                                                         // auctioneer address
172                                                         // padding
;
