use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{hash::Hash, signature::Keypair, transport::TransportError};
use soldb_program::id as program_id;

pub async fn setup() -> Result<(BanksClient, Keypair, Hash), TransportError> {
    let program_test = ProgramTest::new("soldb_program", program_id(), None);

    Ok(program_test.start().await)
}
