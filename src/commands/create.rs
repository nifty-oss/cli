use super::*;

pub struct CreateArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub name: String,
    pub asset_keypair_path: Option<PathBuf>,
    pub immutable: bool,
    pub owner: Option<Pubkey>,
    pub priority: Priority,
}

pub fn handle_create(args: CreateArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let asset_sk = if let Some(path) = args.asset_keypair_path {
        read_keypair_file(path).expect("failed to read keypair file")
    } else {
        Keypair::new()
    };
    let authority_sk = config.keypair;

    let asset = asset_sk.pubkey();
    let authority = authority_sk.pubkey();
    let owner = args.owner.unwrap_or(authority);

    let ix_args = CreateInstructionArgs {
        name: args.name,
        standard: Standard::NonFungible,
        mutable: !args.immutable,
        extensions: None,
    };

    let ix = Create {
        asset,
        authority: (authority, false),
        owner,
        payer: Some(authority),
        group: None,
        group_authority: None,
        system_program: Some(system_program::id()),
    }
    .instruction(ix_args);

    let signers = vec![&authority_sk, &asset_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units =
        get_compute_units(&config.client, &[ix.clone()], &signers)?.unwrap_or(200_000);

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx(&config.client, &signers, &instructions)?;

    println!("Asset {asset} created in tx: {sig}");

    Ok(())
}
