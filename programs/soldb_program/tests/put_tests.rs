mod utils;

use borsh::BorshSerialize;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signer::Signer,
    transaction::Transaction,
    transport::TransportError,
};
use solana_system_interface::program;
use soldb_program::{
    id as program_id,
    instructions::{Put, SolDbIntructions},
};
use utils::setup;

#[tokio::test]
async fn test_put() -> Result<(), TransportError> {
    let (banks_client, payer, last_blockhash) = setup().await?;

    // Creates the account
    let name = "Test".to_string();
    let (pda_table_pubkey, table_bump) =
        utils::init_table(&banks_client, &payer, last_blockhash, name).await?;

    let key: Vec<u8> = "k-0".into();
    let value: Vec<u8> = "v-0".into();

    let (pda_val_pubkey, value_bump) = utils::insert(
        &banks_client,
        &payer,
        last_blockhash,
        &pda_table_pubkey,
        key.into(),
        value.clone(),
    )
    .await?;

    let instr = SolDbIntructions::Put(Put {
        table: "Test".to_string(),
        table_bump,
        key: "k-0".into(),
        key_bump: value_bump,
        payload: "v-10".into(),
    });
    let mut ix_data = Vec::new();
    instr.serialize(&mut ix_data).unwrap();

    let ix = Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda_table_pubkey, false),
            AccountMeta::new(pda_val_pubkey, false),
            AccountMeta::new_readonly(program::ID, false),
        ],
        data: ix_data,
    };

    let txn =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], last_blockhash);

    let result = banks_client.process_transaction_with_metadata(txn).await?;

    let meta = result.metadata.expect("should have metadata");
    let logs = meta.log_messages;

    assert!(
        logs.iter().any(|line| line.contains("Put")),
        "expected to find our `msg!` output in logs: {:#?}",
        logs
    );

    Ok(())
}
