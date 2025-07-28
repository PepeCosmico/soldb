#![allow(unexpected_cfgs)]

use crate::processor::process_instruction;
use solana_program::{declare_id, entrypoint};

pub mod accounts;
pub mod error;
pub mod instructions;
pub mod processor;

declare_id!("SDBPbpwuFzj8zjhf4LjQJwYoy2SAJETeBDGKb8keRpq");

entrypoint!(process_instruction);
