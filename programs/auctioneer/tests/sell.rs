use solana_program_test::tokio;
use std::time::SystemTime;

mod utils;
use utils::setup_functions::*;

#[tokio::test]
async fn sell_success() {
    let mut context = auctioneer_program_test().start_with_context().await;

    let (_, auction_house, auction_house_data) = create_auction_house(&mut context, 100, false)
        .await
        .expect("Failed to create Auction House");

    let token = create_nft(&mut context, None)
        .await
        .expect("Failed to create NFT");

    let (sell_accounts, sell_tx) = sell(
        &mut context,
        &auction_house,
        &auction_house_data,
        &token,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - 60) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
        None,
        None,
        None,
        None,
    );
    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .expect("Failed to sell NFT");

    let seller_trade_state_account = context
        .banks_client
        .get_account(sell_accounts.seller_trade_state)
        .await
        .expect("Account not found")
        .expect("Account is empty");

    assert_eq!(seller_trade_state_account.data.len(), 1);
}
