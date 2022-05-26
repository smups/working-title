/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.
  
  srvr is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.
  
  You should have received a copy of the GNU General Public License
  along with srvr.  If not, see <http://www.gnu.org/licenses/>.
*/
use super::*;

//Constants for bit-bashing
const CONTINUE_BIT_MASK: i32 = 0x80;
const MAX_BYTES: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCVarInt(i32);

impl MCDataType for MCVarInt {

  fn decode(buf: &mut RawPacketReader) -> Result<MCVarInt, Err> {
    let mut val = 0i32;
    let mut byte_index = 0usize;
    let mut next_byte;

    loop {
      //get next byte
      next_byte = buf.read_bytes(1)[0] as i32;

      //Bit-byte magic, you can figure this one out!
      val |= (next_byte & !CONTINUE_BIT_MASK) << 7 * byte_index;
      
      //If the continue bit is NOT set, we're done!
      if (next_byte & CONTINUE_BIT_MASK) == 0 {break;}

      //If we above the max. size, throw an error!
      if byte_index > MAX_BYTES {return Err(
        MCDataTypeDecodeError("tried to decode varint longer than 5 bytes".to_string())
      )}

      //increment the index
      byte_index += 1;
    }

    Ok(MCVarInt(val))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    let mut val = u32::from_ne_bytes(self.0.to_ne_bytes());
    const U32_BITMASK: u32 = u32::from_ne_bytes(CONTINUE_BIT_MASK.to_ne_bytes());

    let mut has_next_byte = true;

    while has_next_byte {
      let mut next_byte = (val & !U32_BITMASK) as u8;

      val >>= 7;

      has_next_byte = val != 0;

      if has_next_byte {
        next_byte |= U32_BITMASK as u8;
      }

      buf.write_byte(next_byte);
    }
  }
}

impl From<i32> for MCVarInt{
  fn from(val: i32) -> Self {MCVarInt(val)}
}

impl From<MCVarInt> for i32 {
  fn from(val: MCVarInt) -> Self {val.0}
}

#[cfg(test)]
mod mc_varint_test{

  use rand::{self, Rng};

  use crate::{
    correctness_test,
    mc_dtypes::{MCDataType, mc_varint::MCVarInt},
    raw_packet::{RawPacketReader, RawPacketWriter}
  };

  macro_rules! read_test {
    ($bytes:expr, $num:expr) => {
      assert_eq!(
        MCVarInt::decode(
          &mut RawPacketReader::from_raw(($bytes))).unwrap(),
          MCVarInt(($num)
        )
      )
    };
  }

  macro_rules! write_test {
      ($bytes:expr, $num:expr) => {
          let mut buf = RawPacketWriter::new(0);
          MCVarInt::from(($num)).encode(&mut buf);
          assert_eq!(&($bytes), buf.raw_view());
      };
  }

  #[test]
  fn read_test(){
    read_test!(vec![0x00], 0);
    read_test!(vec![0x00], 0);
    read_test!(vec![0x01], 1);
    read_test!(vec![0x02], 2);
    read_test!(vec![0x7f], 127);
    read_test!(vec![0x80,0x01], 128);
    read_test!(vec![0xff,0x01], 255);
    read_test!(vec![0xdd,0xc7,0x01], 25565);
    read_test!(vec![0xff,0xff,0x7f], 2097151);
    read_test!(vec![0xff,0xff,0xff,0xff,0x07], 2147483647);
    read_test!(vec![0xff,0xff,0xff,0xff,0x0f], -1);
    read_test!(vec![0x80,0x80,0x80,0x80,0x08], -2147483648);
  }

  #[test]
  fn write_test() {
    write_test!(vec![0x00], 0);
    write_test!(vec![0x00], 0);
    write_test!(vec![0x01], 1);
    write_test!(vec![0x02], 2);
    write_test!(vec![0x7f], 127);
    write_test!(vec![0x80,0x01], 128);
    write_test!(vec![0xff,0x01], 255);
    write_test!(vec![0xdd,0xc7,0x01], 25565);
    write_test!(vec![0xff,0xff,0x7f], 2097151);
    write_test!(vec![0xff,0xff,0xff,0xff,0x07], 2147483647);
    write_test!(vec![0xff,0xff,0xff,0xff,0x0f], -1);
    write_test!(vec![0x80,0x80,0x80,0x80,0x08], -2147483648);
  }

  #[test]
  fn correctness_test() {
    let mut rng = rand::thread_rng();
    //Try 100 random bytes
    for _ in 0..100 {
      let num: i32 = rng.gen();
      correctness_test!(crate::mc_dtypes::MCVarInt, num);
    }
  }
  
}