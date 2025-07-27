use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use solana_system_interface::instruction;

use crate::instructions::{CreateTable, Delete, Put, SolDbIntructions};
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolDbIntructions::unpack(instruction_data)?;

    match instruction {
        SolDbIntructions::CreateTable(create_table) => {
            process_create_table(create_table, program_id, accounts)?;
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

fn process_create_table(
    create_table: CreateTable,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let owner_info = next_account_info(account_iter)?;
    let pda_info = next_account_info(account_iter)?;
    let sys_prog = next_account_info(account_iter)?;

    let (expected_pda, expected_bump) = Pubkey::find_program_address(
        &[&create_table.name.as_ref(), owner_info.key.as_ref()],
        program_id,
    );

    if pda_info.key != &expected_pda || create_table.bump != expected_bump {
        msg!("PDA mismatch");
        return Err(ProgramError::InvalidSeeds);
    }

    let rent = Rent::get()?;
    let space: u64 = 0;
    let lamports = rent.minimum_balance(space as usize);

    let seeds = &[
        create_table.name.as_ref(),
        owner_info.key.as_ref(),
        &[create_table.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let ix = instruction::create_account(owner_info.key, pda_info.key, lamports, space, program_id);
    invoke_signed(
        &ix,
        &[owner_info.clone(), pda_info.clone(), sys_prog.clone()],
        signer_seeds,
    )?;

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
