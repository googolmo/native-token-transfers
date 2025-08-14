use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_info::AccountInfo;
use solana_instruction::AccountMeta;
use solana_program_error::{ProgramResult};
use super::Finality;

#[derive(BorshDeserialize, BorshSerialize)]
/// Wormhole instructions.
pub enum Instruction {
    Initialize, // placeholder
    PostMessage {
        batch_id: u32,
        payload: Vec<u8>,
        finality: Finality,
    },
    PostVAA {
        version: u8,
        guardian_set_index: u32,
        timestamp: u32,
        nonce: u32,
        emitter_chain: u16,
        emitter_address: [u8; 32],
        sequence: u64,
        consistency_level: u8,
        payload: Vec<u8>,
    },
    SetFees,            // placeholder (governance action)
    TransferFees,       // placeholder (governance action)
    UpgradeContract,    // placeholder (governance action)
    UpgradeGuardianSet, // placeholder (governance action)
    VerifySignatures {
        signers: [i8; 19],
    },
    PostMessageUnreliable, // placeholder (unused)
}

pub struct PostMessageAccount<'info, 'b> {
    pub config: &'b AccountInfo<'info>,
    pub message: &'b AccountInfo<'info>,
    pub emitter: &'b AccountInfo<'info>,
    pub sequence: &'b AccountInfo<'info>,
    pub payer: &'b AccountInfo<'info>,
    pub fee_collector: &'b AccountInfo<'info>,
    pub clock: &'b AccountInfo<'info>,
    pub rent: &'b AccountInfo<'info>,
    pub system_program: &'b AccountInfo<'info>,
}

pub struct PostMessageCpi<'info, 'b> {
  pub __program: &'b AccountInfo<'info>,
  pub config: &'b AccountInfo<'info>,
  pub message: &'b AccountInfo<'info>,
  pub emitter: &'b AccountInfo<'info>,
  pub sequence: &'b AccountInfo<'info>,
  pub payer: &'b AccountInfo<'info>,
  pub fee_collector: &'b AccountInfo<'info>,
  pub clock: &'b AccountInfo<'info>,
  pub rent: &'b AccountInfo<'info>,
  pub system_program: &'b AccountInfo<'info>,
  pub msg: Vec<u8>,
}

impl<'a, 'b> PostMessageCpi<'a, 'b> {
  pub fn new(
    program: &'b AccountInfo<'a>,
    accounts: PostMessageAccount<'a, 'b>,
    batch_id: u32,
    payload: Vec<u8>,
    finality: Finality,
  ) -> Self {
    Self {
      __program: program,
      config: accounts.config,
      message: accounts.message,
      emitter: accounts.emitter,
      sequence: accounts.sequence,
      payer: accounts.payer,
      fee_collector: accounts.fee_collector,
      clock: accounts.clock,
      rent: accounts.rent,
      system_program: accounts.system_program,
      msg: borsh::to_vec(&Instruction::PostMessage { batch_id, payload, finality }).unwrap(),
    }
  }

  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> ProgramResult{
    let ix = solana_instruction::Instruction {
        program_id: *self.__program.key,
        accounts: vec![
            AccountMeta::new(*self.config.key, false),
            AccountMeta::new(*self.message.key, true),
            AccountMeta::new_readonly(*self.emitter.key, true),
            AccountMeta::new(*self.sequence.key, false),
            AccountMeta::new(*self.payer.key, true),
            AccountMeta::new(*self.fee_collector.key, false),
            AccountMeta::new_readonly(*self.clock.key, false),
            AccountMeta::new_readonly(*self.system_program.key, false),
            AccountMeta::new_readonly(*self.rent.key, false),
        ],
        data: self.msg.clone(),
    };

    solana_cpi::invoke_signed(
        &ix,
        &[
           self.__program.clone(),
           self.config.clone(),
           self.message.clone(),
           self.emitter.clone(),
           self.sequence.clone(),
           self.payer.clone(),
           self.fee_collector.clone(),
           self.clock.clone(),
           self.system_program.clone(),
           self.rent.clone(),
        ],
        signers_seeds,
    )
  }
}


//
// pub fn post_message<'info>(
//     ctx: CpiContext<'_, '_, '_, 'info, PostMessage<'info>>,
//     batch_id: u32,
//     payload: Vec<u8>,
//     finality: Finality,
// ) -> Result<()> {
//     let ix = solana_instruction::Instruction {
//         program_id: ctx.program.key(),
//         accounts: vec![
//             AccountMeta::new(ctx.accounts.config.key(), false),
//             AccountMeta::new(ctx.accounts.message.key(), true),
//             AccountMeta::new_readonly(ctx.accounts.emitter.key(), true),
//             AccountMeta::new(ctx.accounts.sequence.key(), false),
//             AccountMeta::new(ctx.accounts.payer.key(), true),
//             AccountMeta::new(ctx.accounts.fee_collector.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.clock.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
//             AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
//         ],
//         data: Instruction::PostMessage {
//             batch_id,
//             payload,
//             finality,
//         }
//         .try_to_vec()?,
//     };
//
//     solana_cpi::invoke_signed(
//         &ix,
//         &ToAccountInfos::to_account_infos(&ctx),
//         ctx.signer_seeds,
//     )
//     .map_err(Into::into)
// }

