use crate::error::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum SolDbIntructions {
    CreateTable(CreateTable),
    Put(Put),
    Delete(Delete),
}

impl SolDbIntructions {
    pub fn unpack(input: &[u8]) -> Result<Self> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => {
                let create_table = CreateTable::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::CreateTable(create_table))
            }
            1 => {
                let put =
                    Put::try_from_slice(rest).map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::Put(put))
            }
            2 => {
                let delete = Delete::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::Delete(delete))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreateTable {
    pub name: String,
    pub bump: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Put {
    pub key: Vec<u8>,
    pub payload: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Delete {
    pub key: Vec<u8>,
}
