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

  fn decode(buf: &[u8]) -> Result<MCVarInt, Err> {
    let mut val = 0i32;
    let mut byte_index = 0usize;

    loop {
      //Bit-byte magic, you can figure this one out!
      val |= (buf[byte_index] as i32 & !CONTINUE_BIT_MASK) << 7 * byte_index;
      
      //If the continue bit is NOT set, we're done!
      if (buf[byte_index] as i32 & CONTINUE_BIT_MASK) == 0 {break;}

      //If we above the max. size, throw an error!
      if byte_index > MAX_BYTES {return Err(
        MCDataTypeDecodeError("tried to decode varint longer than 5 bytes".to_string())
      )}

      //increment the index
      byte_index += 1;
    }

    Ok(MCVarInt(val))
  }

  fn encode(&self, buf: &mut [u8]) {
    let mut val = u32::from_le_bytes(self.0.to_le_bytes());
    let mut byte_index = 0usize;

    loop {

      if (val & CONTINUE_BIT_MASK as u32) == 0 {
        buf[byte_index] = val.to_le_bytes()[0]; //least significant byte
        return;
      }

      buf[byte_index] = (
        (val & !(CONTINUE_BIT_MASK as u32)) | CONTINUE_BIT_MASK as u32
      ).to_le_bytes()[0];

      val = val >> 7;
      byte_index += 1;
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

  use crate::mc_dtypes::{MCDataType, mc_varint::MCVarInt};

  #[test]
  fn read_test(){
    assert_eq!(MCVarInt::decode(&[0x00]).unwrap(), MCVarInt(0));
    assert_eq!(MCVarInt::decode(&[0x01]).unwrap(), MCVarInt(1));
    assert_eq!(MCVarInt::decode(&[0x02]).unwrap(), MCVarInt(2));
    assert_eq!(MCVarInt::decode(&[0x7f]).unwrap(), MCVarInt(127));
    assert_eq!(MCVarInt::decode(&[0x80,0x01]).unwrap(), MCVarInt(128));
    assert_eq!(MCVarInt::decode(&[0xff,0x01]).unwrap(), MCVarInt(255));
    assert_eq!(MCVarInt::decode(&[0xdd,0xc7,0x01]).unwrap(), MCVarInt(25565));
    assert_eq!(MCVarInt::decode(&[0xff,0xff,0x7f]).unwrap(), MCVarInt(2097151));
    assert_eq!(MCVarInt::decode(&[0xff,0xff,0xff,0xff,0x07]).unwrap(), MCVarInt(2147483647));
    assert_eq!(MCVarInt::decode(&[0xff,0xff,0xff,0xff,0x0f]).unwrap(), MCVarInt(-1));
    assert_eq!(MCVarInt::decode(&[0x80,0x80,0x80,0x80,0x08]).unwrap(), MCVarInt(-2147483648));
  }

  #[test]
  fn write_test() {
    let mut buf = [0,0,0,0,0];

    MCVarInt::from(0).encode(&mut buf);
    assert_eq!([0u8,0,0,0,0], buf);

    MCVarInt::from(1).encode(&mut buf);
    assert_eq!([0x01,0,0,0,0], buf);

    MCVarInt::from(2).encode(&mut buf);
    assert_eq!([0x02,0,0,0,0], buf);

    MCVarInt::from(127).encode(&mut buf);
    assert_eq!([0x7f,0,0,0,0], buf);

    MCVarInt::from(128).encode(&mut buf);
    assert_eq!([0x80,0x01,0,0,0], buf);

    MCVarInt::from(255).encode(&mut buf);
    assert_eq!([0xff,0x01,0,0,0], buf);

    MCVarInt::from(25565).encode(&mut buf);
    assert_eq!([0xdd,0xc7,0x01,0,0], buf);

    MCVarInt::from(2097151).encode(&mut buf);
    assert_eq!([0xff,0xff,0x7f,0,0], buf);

    MCVarInt::from(2147483647).encode(&mut buf);
    assert_eq!([0xff,0xff,0xff,0xff,0x07], buf);

    MCVarInt::from(-1).encode(&mut buf);
    assert_eq!([0xff,0xff,0xff,0xff,0x0f], buf);

    MCVarInt::from(-2147483648).encode(&mut buf);
    assert_eq!([0x80,0x80,0x80,0x80,0x08], buf);
  }
}