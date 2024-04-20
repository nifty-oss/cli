use nifty_asset::instructions::Transfer;

use super::*;

pub struct TransferArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub asset: Pubkey,
    pub recipient: Pubkey,
    pub priority: Priority,
}

pub fn handle_transfer(args: TransferArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let signer_sk = config.keypair;

    let signer = signer_sk.pubkey();
    let asset = args.asset;
    let recipient = args.recipient;

    let ix = Transfer {
        asset,
        signer,
        recipient,
        group: None,
    }
    .instruction();

    let signers = vec![&signer_sk];

    let micro_lamports = get_priority_fee(&args.priority);
    let compute_units = get_compute_units(&config.client, &[ix.clone()], &signers)?;

    let instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_units as u32),
        ComputeBudgetInstruction::set_compute_unit_price(micro_lamports),
        ix,
    ];

    let sig = send_and_confirm_tx_with_spinner(&config.client, &signers, &instructions)?;

    println!("Transferring asset {asset} to {recipient} in tx: {sig}");

    Ok(())
}
