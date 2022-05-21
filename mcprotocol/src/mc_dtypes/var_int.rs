//Variable length integer

use super::MCDataType;

const SEGMENT_BIT_MASK: i32 = 0x7F;
const CONTINUE_BIT_MASK: i32 = 0x80;
const MAX_SIZE: i32 = 32;

#[derive(Debug)]
pub enum MCVarInt{
    Valid(i32),
    Corrupted
}

impl MCDataType for MCVarInt {
    fn decode(buf: &Vec<u8>) -> Self {
        use MCVarInt::*;

        let mut value = 0i32;
        let mut pos = 0i32;

        let mut counter = 0usize;
        let mut current_byte;

        loop {
            current_byte = buf[counter] as i32;
            value |= (current_byte & SEGMENT_BIT_MASK) << pos;
            pos += 7;

            //Break if we do NOT find the continue bit
            if (current_byte & CONTINUE_BIT_MASK) == 0 {break;}

            pos += 7; //Shift over 7 bits

            //Return a corrupted variant if the varint is too large
            if pos >= MAX_SIZE {return Corrupted};

            counter += 1;
        }

        Valid(value)
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        use MCVarInt::*;
        match self {
            Corrupted => panic!(),
            Valid(val) => {
                loop {
                    if (val & !SEGMENT_BIT_MASK) == 0 {
                        buf.push(*val)
                    }
                }
            }
        }
    }
}