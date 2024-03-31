use crate::transaction::pack_instructions;

use super::*;

pub struct MintArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset_file_path: PathBuf,
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

    let instructions = mint(MintIxArgs {
        accounts,
        asset_args,
        extension_args,
    })?;

    let packed_instructions = pack_instructions(2, &authority_sk.pubkey(), &instructions);

    // Instructions are packed to max data length sizes, so we only put one in each tx.
    for instructions in packed_instructions {
        let sig = send_and_confirm_tx(&config.client, &[&authority_sk, &asset_sk], &instructions)?;
        println!("sig: {}", sig);
    }

    println!("Mint asset: {}", asset);

    Ok(())
}
