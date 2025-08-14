use borsh::BorshDeserialize;
use solana_program_pack::Pack;

/// Anchor does not have a SPL Multisig wrapper as a part of the token interface:
/// https://docs.rs/anchor-spl/0.29.0/src/anchor_spl/token_interface.rs.html
/// Thus, we have to write our own wrapper to use with `InterfaceAccount`

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SplMultisig{
  pub discriminator: [u8;8],
  pub multi_sig: spl_token_2022::state::Multisig,
}

impl BorshDeserialize for SplMultisig {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
       let discriminator = <[u8; 8]>::deserialize_reader(reader)?;

        // Read the multisig data
        let mut multisig_data = vec![0u8; spl_token_2022::state::Multisig::LEN];
        reader.read_exact(&mut multisig_data)?;

        let multi_sig = spl_token_2022::state::Multisig::unpack(&multisig_data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
 
        Ok(SplMultisig {
            discriminator,
            multi_sig,
        })
    }
}
