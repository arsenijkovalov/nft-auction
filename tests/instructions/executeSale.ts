import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Auctioneer } from "../../target/types/auctioneer";
import * as pda from "../pda";
import {
  NFT,
  AuctionHouseData,
  SellAccounts,
  BuyAccounts,
  Creator,
} from "../interfaces";
import { AUCTION_HOUSE_PROGRAM_ID } from "../generated";

export async function executeSale({
  auctioneerProgram,
  auctionHouse,
  token,
  buyer,
  buyerPrice,
  sellAccounts,
  buyAccounts,
  creators,
  tokenSize,
}: {
  auctioneerProgram: Program<Auctioneer>;
  auctionHouse: AuctionHouseData;
  token: NFT;
  buyer: anchor.web3.PublicKey;
  buyerPrice: number;
  sellAccounts: SellAccounts;
  buyAccounts: BuyAccounts;
  creators?: Creator[];
  tokenSize: number;
}) {
  const [auctioneerAuthorityAddress, auctioneerAuthorityBump] =
    pda.findAuctioneerAuthorityAddress({
      auctionHouseAddress: auctionHouse.address,
    });
  const [auctioneerAddress] = pda.findAuctioneerAddress({
    auctionHouseAddress: auctionHouse.address,
    auctioneerAuthorityAddress,
  });

  const buyerReceiptTokenAccount = anchor.utils.token.associatedAddress({
    mint: token.mint.publicKey,
    owner: buyer,
  });

  const [, freeSellerTradeStateBump] = pda.findTradeStateAddress({
    wallet: token.owner.publicKey,
    auctionHouseAddress: auctionHouse.address,
    tokenAccount: sellAccounts.tokenAccount,
    treasuryMint: auctionHouse.treasuryMint,
    tokenMint: token.mint.publicKey,
    price: 0,
    tokenSize,
  });
  const [, programAsSignerBump] = pda.findProgramAsSignerAddress();
  const [, escrowBump] = pda.findEscrowPaymentAccountAddress({
    wallet: buyer,
    auctionHouseAddress: auctionHouse.address,
  });

  const remainingAccounts: anchor.web3.AccountMeta[] = []; // NFT creators

  if (creators) {
    for (const creator of creators) {
      remainingAccounts.push({
        pubkey: creator.address,
        isWritable: true,
        isSigner: false,
      });
    }
  }

  const executeSellTx = await auctioneerProgram.methods
    .executeSale(
      escrowBump,
      freeSellerTradeStateBump,
      programAsSignerBump,
      auctioneerAuthorityBump,
      new BN(buyerPrice),
      new BN(tokenSize)
    )
    .accounts({
      auctionHouseProgram: AUCTION_HOUSE_PROGRAM_ID,
      listingConfig: sellAccounts.listingConfig,
      buyer,
      seller: token.owner.publicKey,
      tokenAccount: sellAccounts.tokenAccount,
      tokenMint: token.mint.publicKey,
      metadata: sellAccounts.metadata,
      treasuryMint: auctionHouse.treasuryMint,
      escrowPaymentAccount: buyAccounts.escrowPaymentAccount,
      sellerPaymentReceiptAccount: token.owner.publicKey,
      buyerReceiptTokenAccount,
      authority: auctionHouse.authority,
      auctionHouse: auctionHouse.address,
      auctionHouseFeeAccount: auctionHouse.auctionHouseFeeAccount,
      auctionHouseTreasury: auctionHouse.auctionHouseTreasury,
      buyerTradeState: buyAccounts.buyerTradeState,
      sellerTradeState: sellAccounts.sellerTradeState,
      freeTradeState: sellAccounts.freeSellerTradeState,
      auctioneerAuthority: auctioneerAuthorityAddress,
      auctioneer: auctioneerAddress,
      programAsSigner: sellAccounts.programAsSigner,
    })
    .remainingAccounts(remainingAccounts)
    .rpc();

  console.log("Transaction [Execute Sell]", executeSellTx);
}
