use ntt_messages2::errors::ScalingError;
use solana_program_error::ProgramError;
use thiserror::Error;
use num_derive::FromPrimitive;


#[derive(PartialEq, Clone, Debug, Eq, Error, FromPrimitive)]
pub enum NTTError {
    #[error("CantReleaseYet")]
    CantReleaseYet,
    #[error("InvalidPendingOwner")]
    InvalidPendingOwner,
    #[error("InvalidChainId")]
    InvalidChainId,
    #[error("InvalidRecipientAddress")]
    InvalidRecipientAddress,
    #[error("InvalidTransceiverPeer")]
    InvalidTransceiverPeer,
    #[error("InvalidNttManagerPeer")]
    InvalidNttManagerPeer,
    #[error("InvalidRecipientNttManager")]
    InvalidRecipientNttManager,
    #[error("TransferAlreadyRedeemed")]
    TransferAlreadyRedeemed,
    #[error("TransferCannotBeRedeemed")]
    TransferCannotBeRedeemed,
    #[error("TransferNotApproved")]
    TransferNotApproved,
    #[error("MessageAlreadySent")]
    MessageAlreadySent,
    #[error("InvalidMode")]
    InvalidMode,
    #[error("InvalidMintAuthority")]
    InvalidMintAuthority,
    #[error("TransferExceedsRateLimit")]
    TransferExceedsRateLimit,
    #[error("Paused")]
    Paused,
    #[error("DisabledTransceiver")]
    DisabledTransceiver,
    #[error("InvalidDeployer")]
    InvalidDeployer,
    #[error("BadAmountAfterTransfer")]
    BadAmountAfterTransfer,
    #[error("BadAmountAfterBurn")]
    BadAmountAfterBurn,
    #[error("ZeroThreshold")]
    ZeroThreshold,
    #[error("OverflowExponent")]
    OverflowExponent,
    #[error("OverflowScaledAmount")]
    OverflowScaledAmount,
    #[error("BitmapIndexOutOfBounds")]
    BitmapIndexOutOfBounds,
    #[error("NoRegisteredTransceivers")]
    NoRegisteredTransceivers,
    #[error("NotPaused")]
    NotPaused,
    #[error("InvalidPendingTokenAuthority")]
    InvalidPendingTokenAuthority,
    #[error("IncorrectRentPayer")]
    IncorrectRentPayer,
    #[error("InvalidMultisig")]
    InvalidMultisig,
    #[error("ThresholdTooHigh")]
    ThresholdTooHigh,
    #[error("InvalidTransceiverProgram")]
    InvalidTransceiverProgram,
}

impl From<ScalingError> for NTTError {
    fn from(e: ScalingError) -> Self {
        match e {
            ScalingError::OverflowScaledAmount => NTTError::OverflowScaledAmount,
            ScalingError::OverflowExponent => NTTError::OverflowExponent,
        }
    }
}

impl From<NTTError> for ProgramError {
    fn from(e: NTTError) -> Self {
      ProgramError::Custom(e as u32)
    }
}
