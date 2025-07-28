mod utils;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
    transport::TransportError,
};
use solana_system_interface::program;

use soldb_program::{accounts::SolTable, instructions::SolDbIntructions};

#[tokio::test]
async fn test_create_table() -> Result<(), TransportError> {
    let (banks_client, payer, last_blockhash) = utils::setup().await?;
    let program_id = soldb_program::id();

    let name = "Test".to_string();
    let (pda_pubkey, bump) =
        Pubkey::find_program_address(&[name.as_bytes(), payer.pubkey().as_ref()], &program_id);

    let instr = SolDbIntructions::InitTable(soldb_program::instructions::InitTable {
        name: name.clone(),
        bump,
    });
    let mut ix_data = Vec::new();
    instr.serialize(&mut ix_data).unwrap();

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pda_pubkey, false),
        AccountMeta::new_readonly(program::id(), false),
    ];

    let ix = Instruction {
        program_id: soldb_program::id(),
        accounts,
        data: ix_data,
    };

    let txn =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], last_blockhash);

    banks_client.process_transaction_with_metadata(txn).await?;

    let sol_table = SolTable { name: name.clone() };
    let mut serialized = Vec::new();
    sol_table.serialize(&mut serialized)?;

    let expected_space = serialized.len();
    let rent = banks_client
        .get_rent()
        .await
        .map_err(|_| TransportError::Custom("Error getting Rent".to_string()))?;
    let min_lamports = rent.minimum_balance(expected_space);

    let maybe_account = banks_client.get_account(pda_pubkey).await?;
    assert!(maybe_account.is_some(), "PDA account was not created");

    let account = maybe_account.unwrap();
    assert_eq!(
        account.owner, program_id,
        "PDA account is not owned by the program"
    );
    assert!(
        account.lamports >= min_lamports,
        "PDA account has insufficient lamports ({} < {})",
        account.lamports,
        min_lamports
    );
    assert_eq!(
        account.data.len(),
        expected_space,
        "PDA account data length mismatch"
    );

    let sol_table = SolTable::deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(sol_table.name, name);

    Ok(())
}
