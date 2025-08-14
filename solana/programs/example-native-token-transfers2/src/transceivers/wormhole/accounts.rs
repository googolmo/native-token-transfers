use solana_account_info::AccountInfo;
use solana_cpi::invoke;
use solana_program_error::ProgramResult;
// use wormhole_anchor_sdk::wormhole;
use wormhole_io::TypePrefixedPayload;
use crate::wormhole;

cfg_if::cfg_if! {
    if #[cfg(feature = "tilt-devnet2")] {
        const FINALITY: wormhole::Finality = wormhole::Finality::Confirmed;
    } else if #[cfg(feature = "tilt-devnet")] {
        const FINALITY: wormhole::Finality = wormhole::Finality::Confirmed;
    } else {
        const FINALITY: wormhole::Finality = wormhole::Finality::Finalized;
    }
}

// // TODO: should we add emitter in here too?
// #[derive(Accounts)]
// pub struct WormholeAccounts<'info> {
//     // wormhole stuff
//     #[account(mut)]
//     /// CHECK: address will be checked by the wormhole core bridge
//     pub bridge: Account<'info, wormhole::BridgeData>,
//
//     #[account(mut)]
//     /// CHECK: account will be checked by the wormhole core bridge
//     pub fee_collector: UncheckedAccount<'info>,
//
//     #[account(mut)]
//     /// CHECK: account will be checked and maybe initialized by the wormhole core bridge
//     pub sequence: UncheckedAccount<'info>,
//
//     pub program: Program<'info, wormhole::program::Wormhole>,
//
//     pub system_program: Program<'info, System>,
//
//     // legacy
//     pub clock: Sysvar<'info, Clock>,
//     pub rent: Sysvar<'info, Rent>,
// }

/// SECURITY: Owner checks are disabled. Each of [`WormholeAccounts::bridge`], [`WormholeAccounts::fee_collector`],
/// and [`WormholeAccounts::sequence`] must be checked by the Wormhole core bridge.
/// SECURITY: Signer checks are disabled. The only valid sender is the
/// [`wormhole::PostMessage::emitter`], enforced by the [`CpiContext`] below.
pub fn post_message<'a, 'b, A: TypePrefixedPayload>(
    wormhole_program: &'b AccountInfo<'a>,
    config: &'b AccountInfo<'a>,
    message: &'b AccountInfo<'a>,
    emitter: &'b AccountInfo<'a>,
    sequence: &'b AccountInfo<'a>,
    payer: &'b AccountInfo<'a>,
    fee_collector: &'b AccountInfo<'a>,
    clock: &'b AccountInfo<'a>,
    rent: &'b AccountInfo<'a>,
    system_program: &'b AccountInfo<'a>,
    emitter_bump: u8,
    payload: &A,
    bridge_data: &wormhole::BridgeData,
    additional_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let batch_id = 0;
    let bridge_fee = bridge_data.fee();

    if bridge_fee > 0 {
      invoke(
          &solana_system_interface::instruction::transfer(payer.key, fee_collector.key, bridge_fee),
          &[
            system_program.clone(),
            payer.clone(),
            fee_collector.clone(),
          ],
      )?;
    }
    let seeds: &[&[&[&[u8]]]] = &[
        &[&[b"emitter".as_slice(), &[emitter_bump]]],
        additional_seeds,
    ];

    wormhole::PostMessageCpi::new(
        wormhole_program, 
        wormhole::PostMessageAccount {
            config,
            message,
            emitter,
            sequence,
            payer,
            fee_collector,
            clock,
            rent,
            system_program,
        },
        batch_id, 
        TypePrefixedPayload::to_vec_payload(payload),
        FINALITY,
    ).invoke_signed(&seeds.concat())
}
