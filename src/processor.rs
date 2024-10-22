use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey, system_instruction
};

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Parse the amount to distribute from the data parameter
    let amount: u64 = data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;

    // Iterate through each account and send the parsed amount of lamports to each
    for account in account_info_iter {
        invoke(
            &system_instruction::transfer(
                payer.key,
                account.key,
                amount,
            ),
            &[
                payer.clone(),
                account.clone(),
                system_program.clone()
            ]
        )?;
    }

    Ok(())
}