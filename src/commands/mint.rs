use solana_sdk::compute_budget::ComputeBudgetInstruction;

use crate::transaction::{get_compute_units, get_priority_fee, pack_instructions, Priority};

use super::*;

pub struct MintArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset_file_path: PathBuf,
    pub priority: Priority,
}

pub async fn handle_mint(args: MintArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let asset_data: AssetFile = serde_json::from_reader(File::open(args.asset_file_path)?)?;

    let asset_sk = if let Some(path) = asset_data.asset_keypair_path {
        read_keypair_file(path).expect("failed to read keypair file")
    } else {
        Keypair::new()
    };
    let authority_sk = config.keypair;

    let asset = asset_sk.pubkey();
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

    let micro_lamports = get_priority_fee(&args.priority);

    let instructions = mint(MintIxArgs {
        accounts,
        asset_args,
        extension_args,
    })?;

    let packed_instructions = pack_instructions(2, &authority_sk.pubkey(), &instructions);

    let signers = vec![&authority_sk, &asset_sk];

    // Instructions are packed to max data length sizes, so we only put one in each tx.
    for instructions in packed_instructions {
        let compute_units = get_compute_units(&config.client, &instructions, &signers)?;

        let mut final_instructions = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
            ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ];
        final_instructions.extend(instructions);

        let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &final_instructions)?;
        println!("sig: {}", sig);
    }

    println!("Minted asset: {}", asset);

    Ok(())
}
