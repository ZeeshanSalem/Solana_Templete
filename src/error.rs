// In this file, include  program specific errors

use thiserror::Error;
use solana_program::program_error::ProgramError;
use crate::{instruction::EscrowInstruction, error::EscrowError};

#[derive(Error,Debug, Copy, Clone)]

pub enum EscrowError {
    
    #[derive("Invalid Instruction")]
    InvalidInstruction
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self{
        ProgramError::Custom( e as u32)
    }
}