mod utils;

use borsh::BorshSerialize;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    signer::Signer,
    sysvar::Sysvar,
    transaction::Transaction,
    transport::TransportError,
};
use solana_system_interface::program;

use soldb_program::instructions::SolDbIntructions;

#[tokio::test]
async fn test_create_table() -> Result<(), TransportError> {
    let (banks_client, payer, last_blockhash) = utils::setup().await?;
    let program_id = soldb_program::id();

    let name = "Test".to_string();
    let (pda_pubkey, bump) =
        Pubkey::find_program_address(&[name.as_bytes(), payer.pubkey().as_ref()], &program_id);

    let instr = SolDbIntructions::CreateTable(soldb_program::instructions::CreateTable {
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

    let expected_space = 0usize;
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

    Ok(())
}
