/* ANCHOR: all */
// ANCHOR: imports
use fuels::{
    crypto::SecretKey,
    accounts::{
        predicate::Predicate,
        wallet::WalletUnlocked,
        Account,
    },
    prelude::*,
    types::transaction_builders::{ScriptTransactionBuilder, BuildableTransaction},
};
// ANCHOR_END: imports

// ANCHOR: predicate_abi
abigen!(Predicate(
    name = "MultiSig",
    abi = "./out/debug/predicate-abi.json"
));
// ANCHOR_END: predicate_abi

// ANCHOR: setup
async fn setup_wallets_and_network() -> (Vec<WalletUnlocked>, Provider, AssetId) {
    // ANCHOR: wallet_setup
    // WALLETS
    let private_key_0: SecretKey =
        "0xc2620849458064e8f1eb2bc4c459f473695b443ac3134c82ddd4fd992bd138fd"
            .parse()
            .unwrap();
    let private_key_1: SecretKey =
        "0x37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd"
            .parse()
            .unwrap();
    let private_key_2: SecretKey =
        "0x976e5c3fa620092c718d852ca703b6da9e3075b9f2ecb8ed42d9f746bf26aafb"
            .parse()
            .unwrap();

    let mut wallet_0: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_0, None);
    let mut wallet_1: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_1, None);
    let mut wallet_2: WalletUnlocked = WalletUnlocked::new_from_private_key(private_key_2, None);
    // ANCHOR_END: wallet_setup

    // ANCHOR: token_setup
    // TOKENS
    let asset_id = AssetId::default();

    let all_coins = [&wallet_0, &wallet_1, &wallet_2]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();
    // ANCHOR_END: token_setup

    // ANCHOR: network_setup
    // NETWORKS
    let node_config = NodeConfig::default();

    let provider = setup_test_provider(all_coins, vec![], Some(node_config), None).await.unwrap();
    // ANCHOR_END: network_setup

    [&mut wallet_0, &mut wallet_1, &mut wallet_2]
        .iter_mut()
        .for_each(|wallet| {
            wallet.set_provider(provider.clone());
        });

    return (
        vec![wallet_0, wallet_1, wallet_2],
        provider,
        asset_id,
    );
}
// ANCHOR_END: setup

// ANCHOR: ordered_two_signatures
#[tokio::test]
async fn multisig_two_of_three() -> Result<()> {
    let (wallets, provider, asset_id) = setup_wallets_and_network().await;

    // ANCHOR: configurables
    // CONFIGURABLES
    let required_signatures = 2;
    let signers: [Address; 3] = [
        wallets[0].address().into(),
        wallets[1].address().into(),
        wallets[2].address().into(),
    ];

    let configurables = MultiSigConfigurables::default()
        .with_REQUIRED_SIGNATURES(required_signatures)?
        .with_SIGNERS(signers)?;
    // ANCHOR_END: configurables

    // ANCHOR: predicate_test
    // PREDICATE
    let predicate_binary_path = "./out/debug/predicate.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(provider.clone())
        // ANCHOR: load_configurables
        .with_configurables(configurables);
        // ANCHOR_END: load_configurables
    // ANCHOR_END: predicate_test
    
    // ANCHOR: fund_predicate
    // FUND PREDICATE
    let multisig_amount = 100;
    let wallet_0_amount = provider.get_asset_balance(wallets[0].address(), asset_id).await?;

    wallets[0]
        .transfer(predicate.address(), multisig_amount, asset_id, TxPolicies::default())
        .await?;
    // ANCHOR_END: fund_predicate

    // ANCHOR: transaction_building
    // BUILD TRANSACTION
    let mut tb: ScriptTransactionBuilder = {
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1, None).await?;
        // ANCHOR: output
        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount - 1);
        // ANCHOR_END: output

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default(),
        )
    };
    // ANCHOR_END: transaction_building

    // ANCHOR: sign_transaction
    // SIGN TRANSACTION
    tb.add_signer(wallets[0].clone())?;
    tb.add_signer(wallets[1].clone())?;
    // ANCHOR_END: sign_transaction

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount - 1);

    // ANCHOR: broadcast_transaction
    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;
    // ANCHOR_END: broadcast_transaction

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, 0);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - 2);

    Ok(())
}
// ANCHOR_END: ordered_two_signatures

// ANCHOR: unordered_three_signatures
#[tokio::test]
async fn multisig_mixed_three_of_three() -> Result<()> {
    let (wallets, provider, asset_id) = setup_wallets_and_network().await;

    // CONFIGURABLES
    let required_signatures = 3;
    let signers: [Address; 3] = [
        wallets[0].address().into(),
        wallets[1].address().into(),
        wallets[2].address().into(),
    ];

    let configurables = MultiSigConfigurables::default()
        .with_REQUIRED_SIGNATURES(required_signatures)?
        .with_SIGNERS(signers)?;

    // PREDICATE
    let predicate_binary_path = "./out/debug/predicate.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(provider.clone())
        .with_configurables(configurables);

    let multisig_amount = 100;
    let wallet_0_amount = provider.get_asset_balance(wallets[0].address(), asset_id).await?;


    wallets[0]
        .transfer(predicate.address(), multisig_amount, asset_id, TxPolicies::default())
        .await?;

    let mut tb: ScriptTransactionBuilder = {
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1, None).await?;

        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount - 1);

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default(),
        )
    };

    // NOTE Cannot be signed in any order
    tb.add_signer(wallets[2].clone())?;
    tb.add_signer(wallets[0].clone())?;
    tb.add_signer(wallets[1].clone())?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount - 1);

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, 0);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - 2);

    Ok(())
}
// ANCHOR_END: unordered_three_signatures

// ANCHOR: not_enough_signatures
#[tokio::test]
async fn multisig_not_enough_signatures_fails() -> Result<()> {
    let (wallets, provider, asset_id) = setup_wallets_and_network().await;

    // CONFIGURABLES
    let required_signatures = 2;
    let signers: [Address; 3] = [
        wallets[0].address().into(),
        wallets[1].address().into(),
        wallets[2].address().into(),
    ];

    let configurables = MultiSigConfigurables::default()
        .with_REQUIRED_SIGNATURES(required_signatures)?
        .with_SIGNERS(signers)?;

    // PREDICATE
    let predicate_binary_path = "./out/debug/predicate.bin";
    let predicate: Predicate = Predicate::load_from(predicate_binary_path)?
        .with_provider(provider.clone())
        .with_configurables(configurables);

    let multisig_amount = 100;
    let wallet_0_amount = provider.get_asset_balance(wallets[0].address(), asset_id).await?;


    wallets[0]
        .transfer(predicate.address(), multisig_amount, asset_id, TxPolicies::default())
        .await?;

    let mut tb: ScriptTransactionBuilder = {
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1, None).await?;

        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount);

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default(),
        )
    };

    tb.add_signer(wallets[0].clone())?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount - 1);

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(provider.clone()).await?;
    let _ = provider.send_transaction_and_await_commit(tx).await.is_err();

    Ok(())
}
// ANCHOR_END: not_enough_signatures
/* ANCHOR_END: all */