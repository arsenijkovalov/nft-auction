use anchor_client::solana_sdk::{signature::Keypair, signer::Signer};
use solana_program_test::tokio;

mod utils;
use utils::setup_functions::*;

#[tokio::test]
async fn deposit_success() {
    let mut context = auctioneer_program_test().start_with_context().await;

    let (_, auction_house, auction_house_data) = create_auction_house(&mut context, 100, false)
        .await
        .expect("Failed to create Auction House");

    let buyer = Keypair::new();
    airdrop(&mut context, &buyer.pubkey(), ONE_SOL * 2)
        .await
        .unwrap();

    let (deposit_accounts, deposit_tx) = deposit(
        &mut context,
        &auction_house,
        &auction_house_data,
        &buyer,
        ONE_SOL,
    );
    context
        .banks_client
        .process_transaction(deposit_tx)
        .await
        .unwrap();

    let escrow_payment_account_data_len = 0;
    let rent = context.banks_client.get_rent().await.unwrap();
    let rent_exempt_min: u64 = rent.minimum_balance(escrow_payment_account_data_len);

    let escrow_payment_account = context
        .banks_client
        .get_account(deposit_accounts.escrow_payment_account)
        .await
        .expect("Account not found")
        .expect("Account is empty");

    assert_eq!(escrow_payment_account.lamports, ONE_SOL + rent_exempt_min);
}
