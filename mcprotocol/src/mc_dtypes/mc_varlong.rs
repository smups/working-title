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
const CONTINUE_BIT_MASK: i64 = 0x80;
const MAX_BYTES: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCVarLong(pub i64);

impl MCDataType for MCVarLong {

  fn decode(buf: &mut RawPacketReader) -> Result<MCVarLong, Err> {
    let mut val = 0i64;
    let mut byte_index = 0usize;
    let mut next_byte;

    loop {
      //get next byte
      next_byte = buf.read_bytes(1)[0] as i64;

      //Bit-byte magic, you can figure this one out!
      val |= (next_byte & !CONTINUE_BIT_MASK) << 7 * byte_index;
      
      //If the continue bit is NOT set, we're done!
      if (next_byte & CONTINUE_BIT_MASK) == 0 {break;}

      //If we above the max. size, throw an error!
      if byte_index > MAX_BYTES {return Err(
        MCDataTypeDecodeError("tried to decode varlong longer than 10 bytes".to_string())
      )}

      //increment the index
      byte_index += 1;
    }

    Ok(MCVarLong(val))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    let mut val = u64::from_ne_bytes(self.0.to_ne_bytes());
    const U64_BITMASK: u64 = u64::from_ne_bytes(CONTINUE_BIT_MASK.to_ne_bytes());

    let mut has_next_byte = true;

    while has_next_byte {
      let mut next_byte = (val & !U64_BITMASK) as u8;

      val >>= 7;

      has_next_byte = val != 0;

      if has_next_byte {
        next_byte |= U64_BITMASK as u8;
      }

      buf.write_byte(next_byte);
    }
  }
}

impl From<i64> for MCVarLong{
  fn from(val: i64) -> Self {MCVarLong(val)}
}

impl From<MCVarLong> for i64 {
  fn from(val: MCVarLong) -> Self {val.0}
}

impl MCVarLong {
  pub fn byte_size(&self) -> usize {todo!()}
}

#[cfg(test)]
mod mc_varlong_test{

  use crate::{
    mc_dtypes::{MCDataType, mc_varlong::MCVarLong},
    raw_packet::{RawPacketReader, RawPacketWriter}
  };

  macro_rules! read_test {
    ($bytes:expr, $num:expr) => {
      assert_eq!(
        MCVarLong::decode(&mut RawPacketReader::from_raw(($bytes))).unwrap(),
        MCVarLong(($num))
      )
    };
  }

  macro_rules! write_test {
      ($bytes:expr, $num:expr) => {
          let mut buf = RawPacketWriter::new(0);
          MCVarLong::from(($num)).encode(&mut buf);
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
    read_test!(vec![0xff,0xff,0xff,0xff,0x07], 2147483647);
    read_test!(vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x7f], 9223372036854775807);
    read_test!(vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x01], -1);
    read_test!(vec![0x80,0x80,0x80,0x80,0xf8,0xff,0xff,0xff,0xff,0x01], -2147483648);
    read_test!(vec![0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x01], -9223372036854775808);
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
    write_test!(vec![0xff,0xff,0xff,0xff,0x07], 2147483647);
    write_test!(vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x7f], 9223372036854775807);
    write_test!(vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x01], -1);
    write_test!(vec![0x80,0x80,0x80,0x80,0xf8,0xff,0xff,0xff,0xff,0x01], -2147483648);
    write_test!(vec![0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x01], -9223372036854775808);
  }
}