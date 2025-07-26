use solana_program::program_error::ProgramError;

pub type Result<T> = std::result::Result<T, ProgramError>;
