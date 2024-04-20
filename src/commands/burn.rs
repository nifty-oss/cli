use super::*;

pub struct BurnArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub recipient: Option<Pubkey>,
    pub priority: Priority,
}

pub fn handle_burn(args: BurnArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;

    let signer = signer_sk.pubkey();
    let asset = args.asset;

    let data = config.client.get_account_data(&args.asset)?;
    let asset_account = Asset::from_bytes(&data).unwrap();

    let ix = Burn {
        asset,
        signer,
        recipient: args.recipient,
        group: asset_account.group.to_option(),
    }
    .instruction();

    let signers = vec![&signer_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units =
        get_compute_units(&config.client, &[ix.clone()], &signers)?.unwrap_or(200_000);

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &instructions)?;

    println!("Burned asset {asset} in tx: {sig}");

    Ok(())
}
