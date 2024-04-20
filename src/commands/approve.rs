use nifty_asset::{
    instructions::{Approve, ApproveInstructionArgs},
    types::{DelegateInput, DelegateRole},
};

use super::*;

pub struct ApproveArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub delegate: Pubkey,
    pub role: Vec<String>,
    pub priority: Priority,
}

pub fn handle_approve(args: ApproveArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let owner_sk = config.keypair;

    let owner = owner_sk.pubkey();
    let asset = args.asset;
    let delegate = args.delegate;

    let roles = args
        .role
        .iter()
        .map(|role| match role.to_lowercase().as_str() {
            "burn" => DelegateRole::Burn,
            "lock" => DelegateRole::Lock,
            "transfer" => DelegateRole::Transfer,
            _ => panic!("Invalid role: {}", role),
        })
        .collect();

    let ix_args = ApproveInstructionArgs {
        delegate_input: DelegateInput::Some { roles },
    };

    let ix = Approve {
        asset,
        owner,
        delegate,
    }
    .instruction(ix_args);

    let signers = vec![&owner_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units =
        get_compute_units(&config.client, &[ix.clone()], &signers)?.unwrap_or(200_000);

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &instructions)?;

    println!("Setting {delegate} as a delegate on asset {asset} in tx: {sig}");

    Ok(())
}
