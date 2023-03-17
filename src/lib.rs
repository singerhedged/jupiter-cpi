anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

pub mod jupiter_override {

    use anchor_lang::prelude::*;
    use anchor_lang::Discriminator;
    use anchor_lang::{AnchorSerialize, InstructionData};
    use std::io::Write;

    use crate::SplitLeg;
    use crate::Swap;

    #[derive(Debug)]
    pub enum SwapLeg {
        Chain { swap_legs: Vec<SwapLeg> },
        Split { split_legs: Vec<SplitLeg> },
        Swap { swap: Swap },
    }

    impl AnchorSerialize for SwapLeg {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
            match self {
                SwapLeg::Chain { swap_legs } => {
                    0u8.serialize(writer)?;
                    swap_legs.serialize(writer)
                }
                SwapLeg::Split { split_legs } => {
                    1u8.serialize(writer)?;
                    split_legs.serialize(writer)
                }
                SwapLeg::Swap { swap } => {
                    2u8.serialize(writer)?;
                    swap.serialize(writer)
                }
            }
        }
    }

    impl AnchorDeserialize for SwapLeg {
        fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
            let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
            let return_value = match variant_idx {
                0u8 => SwapLeg::Chain {
                    swap_legs: borsh::BorshDeserialize::deserialize(buf)?,
                },
                1u8 => SwapLeg::Split {
                    split_legs: borsh::BorshDeserialize::deserialize(buf)?,
                },
                2u8 => SwapLeg::Swap {
                    swap: borsh::BorshDeserialize::deserialize(buf)?,
                },
                _ => {
                    let msg =
                        borsh::maybestd::format!("Unexpected variant index: {:?}", variant_idx);
                    return Err(borsh::maybestd::io::Error::new(
                        borsh::maybestd::io::ErrorKind::InvalidInput,
                        msg,
                    ));
                }
            };
            Ok(return_value)
        }
    }

    #[derive(AnchorSerialize, Debug)]
    pub struct Route {
        pub swap_leg: SwapLeg,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    impl AnchorDeserialize for Route {
        fn deserialize(buf: &mut &[u8]) -> std::result::Result<Route, std::io::Error> {
            if buf.len() < Self::DISCRIMINATOR.len() {
                return Err(std::io::ErrorKind::InvalidData.into());
            }
            let given_disc = &buf[..8];
            if &Self::DISCRIMINATOR != given_disc {
                return Err(std::io::ErrorKind::InvalidData.into());
            };
            let raw_data = &mut &buf[8..];
            Ok(Self {
                swap_leg: borsh::BorshDeserialize::deserialize(raw_data)?,
                in_amount: borsh::BorshDeserialize::deserialize(raw_data)?,
                quoted_out_amount: borsh::BorshDeserialize::deserialize(raw_data)?,
                slippage_bps: borsh::BorshDeserialize::deserialize(raw_data)?,
                platform_fee_bps: borsh::BorshDeserialize::deserialize(raw_data)?,
            })
        }
    }

    impl Discriminator for Route {
        const DISCRIMINATOR: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
    }

    impl InstructionData for Route {}
}

#[cfg(test)]
mod test {
    use anchor_lang::prelude::*;

    use crate::jupiter_override::Route;

    #[test]
    pub fn deserialize_route() -> Result<()> {
        let buf = [
            229, 23, 203, 151, 122, 227, 173, 42, 0, 1, 0, 0, 0, 2, 3, 100, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 50, 0, 0,
        ];

        let route = Route::deserialize(&mut &buf[..])?;

        println!("{:?}", route);

        Ok(())
    }
}
