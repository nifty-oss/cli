use nifty_asset::{
    instructions::{Revoke, RevokeInstructionArgs},
    types::{DelegateInput, DelegateRole},
};

use super::*;

pub struct RevokeArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub role: Vec<String>,
    pub all: bool,
    pub priority: Priority,
}

pub fn handle_revoke(args: RevokeArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;
    let signer = signer_sk.pubkey();
    let asset = args.asset;

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

    let ix_args = RevokeInstructionArgs {
        delegate_input: if args.all {
            DelegateInput::All
        } else {
            DelegateInput::Some { roles }
        },
    };

    let ix = Revoke { asset, signer }.instruction(ix_args);

    let signers = vec![&signer_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units = get_compute_units(&config.client, &[ix.clone()], &signers)?;

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &instructions)?;

    println!("Revoking the delegate on asset {asset} in tx: {sig}");

    Ok(())
}
