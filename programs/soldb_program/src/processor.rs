use borsh::BorshSerialize;
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

use crate::{
    accounts::SolTable,
    instructions::{Delete, InitTable, Put, SolDbIntructions},
};
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolDbIntructions::unpack(instruction_data)?;

    match instruction {
        SolDbIntructions::InitTable(init_table) => {
            process_init_table(init_table, program_id, accounts)?;
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

fn process_init_table(
    init_table: InitTable,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let owner_info = next_account_info(account_iter)?;
    let pda_info = next_account_info(account_iter)?;
    let sys_prog = next_account_info(account_iter)?;

    let (expected_pda, expected_bump) = Pubkey::find_program_address(
        &[&init_table.name.as_ref(), owner_info.key.as_ref()],
        program_id,
    );

    if pda_info.key != &expected_pda || init_table.bump != expected_bump {
        msg!("PDA mismatch");
        return Err(ProgramError::InvalidSeeds);
    }

    let sol_table = SolTable {
        name: init_table.name.clone(),
    };
    let mut serialized = Vec::new();
    sol_table.serialize(&mut serialized)?;
    let space = serialized.len() as u64;
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space as usize);

    let seeds = &[
        init_table.name.as_ref(),
        owner_info.key.as_ref(),
        &[init_table.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let ix = instruction::create_account(owner_info.key, pda_info.key, lamports, space, program_id);
    invoke_signed(
        &ix,
        &[owner_info.clone(), pda_info.clone(), sys_prog.clone()],
        signer_seeds,
    )?;

    let sol_table = SolTable {
        name: init_table.name,
    };

    sol_table.serialize(&mut &mut pda_info.data.borrow_mut()[..])?;

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
