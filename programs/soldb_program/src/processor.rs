use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::{Delete, Put, SolDbIntructions};
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolDbIntructions::unpack(instruction_data)?;

    match instruction {
        SolDbIntructions::CreateTable => {
            process_create_table(accounts)?;
        }
        SolDbIntructions::Put(put) => {
            process_put(put, accounts)?;
        }
        SolDbIntructions::Delete(delete) => {
            process_delete(delete, accounts)?;
        }
    };

    Ok(())
}

fn process_create_table(_accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Create Table");
    Ok(())
}

fn process_put(_put: Put, _accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Put");
    Ok(())
}

fn process_delete(_delete: Delete, _accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Delete");
    Ok(())
}
