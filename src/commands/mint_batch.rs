use std::{sync::Arc, time::Duration};

use crate::transaction::{pack_instructions, DEFAULT_CU};

use super::*;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::sync::Mutex;

pub struct MintBatchArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset_files_dir: PathBuf,
    pub delay: u64,
    pub priority: Priority,
}

pub struct AssetStruct {
    pub asset: AssetFile,
    pub asset_sk: Keypair,
    pub owner: Pubkey,
}

pub struct AssetResult {
    pub asset_pubkey: Pubkey,
    pub asset_name: String,
    pub tx_result: TxResult,
}

pub enum TxResult {
    Success,
    Failure(String),
}

impl TxResult {
    pub fn is_success(&self) -> bool {
        matches!(self, TxResult::Success)
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, TxResult::Failure(_))
    }

    pub fn get_error(&self) -> Option<&str> {
        match self {
            TxResult::Success => None,
            TxResult::Failure(err) => Some(err),
        }
    }
}

pub async fn handle_mint_batch(args: MintBatchArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    // Get all JSON asset files in the directory
    let asset_files = std::fs::read_dir(args.asset_files_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    let authority_sk = config.keypair;

    let mut instructions = vec![];

    let asset_keys = Arc::new(Mutex::new(vec![]));
    let asset_results = Arc::new(Mutex::new(vec![]));

    for asset_file_path in asset_files {
        let asset_data: AssetFile = serde_json::from_reader(File::open(asset_file_path)?)?;

        let asset_sk = if let Some(path) = &asset_data.asset_keypair_path {
            read_keypair_file(path).expect("failed to read keypair file")
        } else {
            Keypair::new()
        };

        let asset = asset_sk.pubkey();

        asset_keys.lock().await.push(asset_sk);

        asset_results.lock().await.push(AssetResult {
            asset_pubkey: asset,
            asset_name: asset_data.name.clone(),
            tx_result: TxResult::Success,
        });

        let owner = asset_data.owner;

        let accounts = MintAccounts {
            asset,
            owner,
            payer: Some(authority_sk.pubkey()),
        };
        let asset_args = AssetArgs {
            name: asset_data.name,
            standard: Standard::NonFungible,
            mutable: asset_data.mutable,
        };

        let extension_args = asset_data
            .extensions
            .iter()
            .map(|extension| ExtensionArgs {
                extension_type: extension.extension_type.clone(),
                data: extension.value.clone().into_data(),
            })
            .collect::<Vec<ExtensionArgs>>();

        instructions.push(mint(MintIxArgs {
            accounts,
            asset_args,
            extension_args,
        })?);
    }

    // Instructions for each asset must be submitted serially, but we can parallelize the minting of
    // the assets themselves.

    let mut futures = vec![];

    let client = Arc::new(config.client);
    let authority_sk = Arc::new(authority_sk);

    let mp: MultiProgress = MultiProgress::new();
    let sty =
        ProgressStyle::with_template("[{percent}%] {bar:40.blue/white} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-");

    let micro_lamports = get_priority_fee(&args.priority);

    for (i, asset_instructions) in instructions.into_iter().enumerate() {
        let client = client.clone();
        let authority_sk = authority_sk.clone();
        let asset_keys = asset_keys.clone();
        let asset_results = asset_results.clone();

        let sty_clone = sty.clone();
        let mp_clone = mp.clone();

        // Each thread concurrently mints a single asset by serially executing all the transactions needed
        // to create the asset and set its extension data.
        futures.push(tokio::spawn(async move {
            // Pack all the instructions for minting this asset into as few transactions as possible.
            let packed_instructions =
                pack_instructions(2, &authority_sk.pubkey(), &asset_instructions);

            // Create a progress bar for each asset w/ the number of transactions to send.
            let pb = mp_clone.add(ProgressBar::new(packed_instructions.len() as u64));
            pb.set_style(sty_clone.clone());

            let asset_sk = &asset_keys.lock().await[i];
            let asset_address = &asset_sk.pubkey();
            pb.set_message(format!("sending transactions for asset {asset_address}"));

            let signers = vec![&authority_sk, asset_sk];

            for instructions in packed_instructions {
                let compute_units =
                    get_compute_units(&client, &instructions, &signers).unwrap_or(DEFAULT_CU);

                let mut final_instructions = vec![
                    ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
                    ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
                ];
                final_instructions.extend(instructions);

                let res = send_and_confirm_tx(&client, &signers, &final_instructions);

                pb.inc(1);

                match res {
                    Ok(_) => continue,
                    Err(err) => {
                        let mut results = asset_results.lock().await;
                        results[i].tx_result = TxResult::Failure(err.to_string());
                        break;
                    }
                }
            }
            pb.finish_with_message(format!("asset {asset_address} minted"));
        }));

        // Sleep for a short time to avoid sending transactions too quickly
        tokio::time::sleep(Duration::from_millis(args.delay)).await;
    }

    for future in futures {
        future.await?;
    }

    let results = asset_results.lock().await;
    for result in results.iter() {
        if result.tx_result.is_failure() {
            println!(
                "Failed to mint asset {}: {}",
                result.asset_name,
                result.tx_result.get_error().unwrap()
            );
        }
    }

    mp.clear().unwrap();

    Ok(())
}
