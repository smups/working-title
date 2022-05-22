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
pub struct MCVarInt(pub i32);

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
    let mut val = self.0;//u32::from_ne_bytes(self.0.to_ne_bytes());

    loop {
      if (val & CONTINUE_BIT_MASK) == 0 {
        buf.write_byte(val.to_le_bytes()[0]); //least significant byte
        return;
      }

      buf.write_byte(
        (val & !CONTINUE_BIT_MASK | CONTINUE_BIT_MASK).to_le_bytes()[0]
      );

      val >>= 7;
    }
  }
}

impl From<i32> for MCVarInt{
  fn from(val: i32) -> Self {MCVarInt(val)}
}

impl From<MCVarInt> for i32 {
  fn from(val: MCVarInt) -> Self {val.0}
}

impl MCVarInt {
  pub fn byte_size(&self) -> usize {todo!()}
}

#[cfg(test)]
mod mc_varint_test{

  use crate::{
    mc_dtypes::{MCDataType, mc_varint::MCVarInt},
    raw_packet::{RawPacketReader, RawPacketWriter}
  };

  macro_rules! make_varint {
    ($x:expr) => {
      MCVarInt::decode(&mut RawPacketReader::from_raw(($x))).unwrap()
    };
  }

  #[test]
  fn read_test(){
    assert_eq!(make_varint!(vec![0x00]), MCVarInt(0));
    assert_eq!(make_varint!(vec![0x01]), MCVarInt(1));
    assert_eq!(make_varint!(vec![0x02]), MCVarInt(2));
    assert_eq!(make_varint!(vec![0x7f]), MCVarInt(127));
    assert_eq!(make_varint!(vec![0x80,0x01]), MCVarInt(128));
    assert_eq!(make_varint!(vec![0xff,0x01]), MCVarInt(255));
    assert_eq!(make_varint!(vec![0xdd,0xc7,0x01]), MCVarInt(25565));
    assert_eq!(make_varint!(vec![0xff,0xff,0x7f]), MCVarInt(2097151));
    assert_eq!(make_varint!(vec![0xff,0xff,0xff,0xff,0x07]), MCVarInt(2147483647));
    assert_eq!(make_varint!(vec![0xff,0xff,0xff,0xff,0x0f]), MCVarInt(-1));
    assert_eq!(make_varint!(vec![0x80,0x80,0x80,0x80,0x08]), MCVarInt(-2147483648));
  }

  #[test]
  fn write_test() {
    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(0).encode(&mut buf);
    assert_eq!(&vec![0], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(1).encode(&mut buf);
    assert_eq!(&vec![0x01], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(2).encode(&mut buf);
    assert_eq!(&vec![0x02], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(127).encode(&mut buf);
    assert_eq!(&vec![0x7f], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(128).encode(&mut buf);
    assert_eq!(&vec![0x80,0x01], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(255).encode(&mut buf);
    assert_eq!(&vec![0xff,0x01], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(25565).encode(&mut buf);
    assert_eq!(&vec![0xdd,0xc7,0x01], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(2097151).encode(&mut buf);
    assert_eq!(&vec![0xff,0xff,0x7f], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(2147483647).encode(&mut buf);
    assert_eq!(&vec![0xff,0xff,0xff,0xff,0x07], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(-1).encode(&mut buf);
    assert_eq!(&vec![0xff,0xff,0xff,0xff,0x0f], buf.raw_view());

    let mut buf = RawPacketWriter::new(0);
    MCVarInt::from(-2147483648).encode(&mut buf);
    assert_eq!(&vec![0x80,0x80,0x80,0x80,0x08], buf.raw_view());
  }
}