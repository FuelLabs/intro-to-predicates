/* ANCHOR: all */
use fuels::{
    prelude::*,
    accounts::fuel_crypto::SecretKey
};

abigen!(Script(
    name = "MultiSigScript",
    abi = "./out/debug/predicate-script-logging-abi.json"
));

#[tokio::test]
async fn script_logs() -> Result<()> {
    // WALLET
    let private_key: SecretKey =
    "0xc2620849458064e8f1eb2bc4c459f473695b443ac3134c82ddd4fd992bd138fd"
        .parse()
        .unwrap();

    let mut wallet: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key, None);

    // TOKENS

    let all_coins = [&wallet]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    // PROVIDER
    let node_config = Config::default();

    let provider = setup_test_provider(all_coins, vec![], Some(node_config), None).await.unwrap();

    [&mut wallet]
        .iter_mut()
        .for_each(|wallet| {
            wallet.set_provider(provider.clone());
        });

    // ANCHOR: logs
    let bin_path = "./out/debug/predicate-script-logging.bin";

    let instance = MultiSigScript::new(wallet.clone(), bin_path);

    let response = instance.main().call().await?;
    
    let logs = response.decode_logs();
    println!("{:?}", logs);
    // ANCHOR_END: logs
    Ok(())
    // Now you have an instance of your contract you can use to test each function
}
/* ANCHOR_END: all */