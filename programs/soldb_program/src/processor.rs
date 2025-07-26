use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::{CreateTable, SolDbIntructions};
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolDbIntructions::unpack(instruction_data)?;

    match instruction {
        SolDbIntructions::CreateTable(create_table) => {
            process_create_table(create_table, accounts)?
        }
        _ => msg!("Intruction send: {:?}", instruction),
    };

    Ok(())
}

fn process_create_table(_create_table: CreateTable, _accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Create Table");
    Ok(())
}
