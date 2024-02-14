use fuels::{
    accounts::{
        predicate::Predicate,
        wallet::WalletUnlocked,
        Account,
    },
    crypto::SecretKey,
    prelude::*,
    types::transaction_builders::{ScriptTransactionBuilder, BuildableTransaction},
};

abigen!(Predicate(
    name = "MultiSig",
    abi = "./out/debug/predicate-abi.json"
));

async fn setup_wallets_and_network() -> (Vec<WalletUnlocked>, Provider, AssetId) {
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

    // TOKENS
    let asset_id = AssetId::default();

    let all_coins = [&wallet_0, &wallet_1, &wallet_2]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    // NETWORKS
    let node_config = Config::default();

    let provider = setup_test_provider(all_coins, vec![], Some(node_config), None).await.unwrap();


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

#[tokio::test]
async fn multisig_two_of_three() -> Result<()> {
    let (wallets, provider, asset_id) = setup_wallets_and_network().await;

    // CONFIGURABLES
    let required_signatures = 2;
    let signers: [Address; 3] = [
        wallets[0].address().into(),
        wallets[1].address().into(),
        wallets[2].address().into(),
    ];

    let configurables = MultiSigConfigurables::new()
        .with_REQUIRED_SIGNATURES(required_signatures)
        .with_SIGNERS(signers);

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
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1).await?;

        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount);

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default(),
        )
    };

    tb.add_signer(wallets[0].clone())?;
    tb.add_signer(wallets[1].clone())?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount);

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(&provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, 0);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount);

    Ok(())
}

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

    let configurables = MultiSigConfigurables::new()
        .with_REQUIRED_SIGNATURES(required_signatures)
        .with_SIGNERS(signers);

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
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1).await?;

        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount);

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default()
        )
    };

    // NOTE Cannot be signed in any order
    tb.add_signer(wallets[2].clone())?;
    tb.add_signer(wallets[0].clone())?;
    tb.add_signer(wallets[1].clone())?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount);

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(&provider.clone()).await?;
    provider.send_transaction_and_await_commit(tx).await?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, 0);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount);

    Ok(())
}

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

    let configurables = MultiSigConfigurables::new()
        .with_REQUIRED_SIGNATURES(required_signatures)
        .with_SIGNERS(signers);

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
        let input_coin = predicate.get_asset_inputs_for_amount(asset_id, 1).await?;

        let output_coin =
            predicate.get_asset_outputs_for_amount(wallets[0].address().into(), asset_id, multisig_amount);

        ScriptTransactionBuilder::prepare_transfer(
            input_coin,
            output_coin,
            TxPolicies::default()
        )
    };

    tb.add_signer(wallets[0].clone())?;

    assert_eq!(provider.get_asset_balance(predicate.address(), asset_id).await?, multisig_amount);
    assert_eq!(provider.get_asset_balance(wallets[0].address(), asset_id).await?, wallet_0_amount - multisig_amount);

    // SPEND PREDICATE
    let tx: ScriptTransaction = tb.build(&provider.clone()).await?;
    let _ = provider.send_transaction_and_await_commit(tx).await.is_err();

    Ok(())
}
