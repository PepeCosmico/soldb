use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SolTable {
    table: BTreeMap<Vec<u8>, Vec<u8>>,
}
