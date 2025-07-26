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
            2 => {
                let put =
                    Put::try_from_slice(rest).map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::Put(put))
            }
            3 => {
                let delete = Delete::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::Delete(delete))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CreateTable {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Put {
    key: Vec<u8>,
    payload: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Delete {
    key: Vec<u8>,
}
