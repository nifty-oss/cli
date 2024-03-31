use anyhow::Result;
use nifty_asset::MAX_TX_SIZE;
use retry::{delay::Exponential, retry};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::Instruction;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

#[macro_export]
macro_rules! transaction {
    ($signers:expr, $instructions:expr, $client:expr) => {
        Transaction::new_signed_with_payer(
            $instructions,
            Some(&$signers[0].pubkey()),
            $signers,
            $client.get_latest_blockhash()?,
        )
    };
}

pub fn send_and_confirm_tx(
    client: &RpcClient,
    signers: &[&Keypair],
    ixs: &[Instruction],
) -> Result<Signature> {
    let tx = transaction!(signers, ixs, client);

    let signature = client.send_and_confirm_transaction(&tx)?;

    Ok(signature)
}

pub fn send_and_confirm_tx_with_retries(
    client: &RpcClient,
    signers: &[&Keypair],
    ixs: &[Instruction],
) -> Result<Signature> {
    let tx = transaction!(signers, ixs, client);

    // Send tx with retries.
    let res = retry(
        Exponential::from_millis_with_factor(250, 2.0).take(3),
        || client.send_and_confirm_transaction_with_spinner(&tx),
    )?;

    Ok(res)
}

pub fn pack_instructions<'a>(
    num_signers: u32,
    payer: &'a Pubkey,
    ixs: &'a [Instruction],
) -> Vec<Vec<Instruction>> {
    let mut instructions = vec![];
    let mut tx_instructions = vec![];

    // 64 bytes for each signature + Message size
    let max_payload_size = MAX_TX_SIZE - std::mem::size_of::<Signature>() * num_signers as usize;

    for ix in ixs {
        tx_instructions.push(ix.clone());
        let tx = Transaction::new_with_payer(tx_instructions.as_slice(), Some(payer));
        let tx_len = bincode::serialize(&tx).unwrap().len();

        if tx_len > max_payload_size {
            let last_ix = tx_instructions.pop().unwrap();
            instructions.push(tx_instructions.clone());
            tx_instructions.clear();
            tx_instructions.push(last_ix);
        }
    }

    instructions
}
