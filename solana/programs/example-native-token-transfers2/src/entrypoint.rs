use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_entrypoint::entrypoint;
use solana_program_error::ProgramResult;
use solana_pubkey::Pubkey;

entrypoint!(process_instruction);

fn process_instruction<'a>(
    _program_id: &Pubkey,
    _accounts: &'a [AccountInfo<'a>],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("hello");
    todo!()
}
