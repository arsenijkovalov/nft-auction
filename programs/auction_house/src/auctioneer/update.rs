use anchor_lang::prelude::*;

use crate::{constants::*, errors::AuctionHouseError, AuctionHouse, Auctioneer};

#[derive(Accounts)]
pub struct UpdateAuctioneer<'info> {
    // Auction House instance PDA account.
    #[account(
        mut,
        seeds = [
            PREFIX.as_bytes(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()
        ],
        bump=auction_house.bump,
        has_one=authority
    )]
    pub auction_house: Account<'info, AuctionHouse>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: The auction house authority can set this to whatever external address they wish.
    /// The auctioneer authority - typically a PDA of the Auctioneer program running this action.
    pub auctioneer_authority: UncheckedAccount<'info>,

    /// The auctioneer PDA owned by Auction House.
    #[account(
        mut,
        seeds = [
            AUCTIONEER.as_bytes(),
            auction_house.key().as_ref(),
            auctioneer_authority.key().as_ref()
        ],
        bump=auctioneer.bump,
        has_one=auctioneer_authority
    )]
    pub auctioneer: Account<'info, Auctioneer>,

    pub system_program: Program<'info, System>,
}

pub fn update_auctioneer<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateAuctioneer<'info>>,
) -> Result<()> {
    let auction_house = &mut ctx.accounts.auction_house;
    if !auction_house.has_auctioneer {
        return Err(AuctionHouseError::AuctionHouseNotDelegated.into());
    }

    let auctioneer = &mut ctx.accounts.auctioneer;
    auctioneer.auctioneer_authority = ctx.accounts.auctioneer_authority.key();
    auctioneer.auction_house = ctx.accounts.auction_house.key();

    Ok(())
}
