use nifty_asset::instructions::Lock;

use super::*;

pub struct LockArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub signer_keypair_path: Option<PathBuf>,
    pub priority: Priority,
}

pub fn handle_lock(args: LockArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let payer_sk = Keypair::from_bytes(&config.keypair.to_bytes())?;

    // Use provided signer keypair, or default to the config keypair.
    let signer_sk = if let Some(signer) = args.signer_keypair_path {
        read_keypair_file(signer)
            .map_err(|err| anyhow!("Failed to read signer keypair file: {}", err))?
    } else {
        Keypair::from_bytes(&config.keypair.to_bytes())?
    };

    let signer = signer_sk.pubkey();
    let asset = args.asset;

    let ix = Lock { asset, signer }.instruction();

    let signers = vec![&payer_sk, &signer_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units =
        get_compute_units(&config.client, &[ix.clone()], &signers)?.unwrap_or(200_000);

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &instructions)?;

    println!("Locking asset {asset} in tx: {sig}");

    Ok(())
}
