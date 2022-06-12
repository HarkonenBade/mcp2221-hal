use std::io::Read;
use packed_struct::PackingResult;
use packed_struct::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstByte<const N: u8>;

impl<const N: u8> PackedStruct for ConstByte<N> {
    type ByteArray = [u8;1];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        Ok([N])
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        Ok(Self)
    }
}

impl<const N: u8> packed_bits::NumberOfBytes for ConstByte<N> {
    type AsBytes = [u8;1];

    fn number_of_bytes() -> usize {
        1
    }
}

impl<const N: u8> packed_bits::NumberOfBits for ConstByte<N> {
    type Bytes = packed_bits::Bytes<1>;

    fn number_of_bits() -> usize {
        8
    }
}

#[derive(PrimitiveEnum, Debug, Copy, Clone, PartialEq, Default)]
pub enum ClkDutyCycle {
    #[default]
    Duty0Per = 0b00,
    Duty25Per = 0b01,
    Duty50Per = 0b10,
    Duty75Per = 0b11,
}

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ClkOutDiv {
    #[packed_field(bits="7")]
    update: bool,
    #[packed_field(bits="6:5")]
    _dc: ReservedZeroes<packed_bits::Bits<2>>,
    #[packed_field(bits="4:3", ty="enum")]
    duty_cycle: ClkDutyCycle,
    #[packed_field(bits="2:0")]
    clk_div: Integer<u8, packed_bits::Bits<3>>,
}

#[derive(PackedStruct, Debug, Copy, Clone, PartialEq, Default)]
#[packed_struct(bit_numbering="msb0", size_bytes="64")]
pub struct SetSRAMSettingsCommand {
    #[packed_field(bytes="0")]
    _command: ConstByte<0x60>,
    #[packed_field(bytes="2")]
    clk_out_div: ClkOutDiv,
    #[packed_field(bytes="3")]
    dac_voltage_ref: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="4")]
    dac_output_val: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="5")]
    adc_voltage_ref: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="6")]
    int_conf: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bits="56")]
    gp_conf_update: bool,
    #[packed_field(bytes="8")]
    gp0: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="9")]
    gp1: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="10")]
    gp2: ReservedZeroes<packed_bits::Bits<8>>,
    #[packed_field(bytes="11")]
    gp3: ReservedZeroes<packed_bits::Bits<8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let msg = SetSRAMSettingsCommand::default();
        println!("{:?}", msg.pack().unwrap());
    }
}