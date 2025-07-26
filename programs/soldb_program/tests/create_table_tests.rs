mod utils;

use borsh::BorshSerialize;
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction, signer::Signer, transaction::Transaction, transport::TransportError,
};

use soldb_program::{id as program_id, instructions::SolDbIntructions};

#[tokio::test]
async fn test_create_table() -> Result<(), TransportError> {
    let (banks_client, payer, last_blockhash) = utils::setup().await?;

    let instr = SolDbIntructions::CreateTable;
    let mut ix_data = Vec::new();
    instr.serialize(&mut ix_data).unwrap();

    let ix = Instruction {
        program_id: program_id(),
        accounts: Vec::new(),
        data: ix_data,
    };

    let txn =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], last_blockhash);

    let result = banks_client.process_transaction_with_metadata(txn).await?;

    let meta = result.metadata.expect("should have metadata");
    let logs = meta.log_messages;

    assert!(
        logs.iter().any(|line| line.contains("Create Table")),
        "expected to find our `msg!` output in logs: {:#?}",
        logs
    );

    Ok(())
}
