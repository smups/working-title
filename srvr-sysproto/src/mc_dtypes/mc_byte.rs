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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCByte(i8);

impl MCDataType for MCByte {

  fn decode(buf: &mut RawPacketReader) -> Result<MCByte, Err> {
    Ok(MCByte(i8::from_be_bytes([buf.read_bytes(1)[0]])))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&i8::to_be_bytes((*self).into()))
  }
}

impl From<i8> for MCByte{
  fn from(val: i8) -> Self {MCByte(val)}
}

impl From<MCByte> for i8 {
  fn from(val: MCByte) -> Self {val.0}
}

#[cfg(test)]
mod mc_byte_test {

  use rand::{self, Rng};

  #[macro_use]
  use crate::correctness_test;

  #[test]
  fn correctness_test() {
    use crate::mc_dtypes::MCByte;
    let mut rng = rand::thread_rng();
    //Try 100 random bytes
    for _ in 0..100 {
      let num: i8 = rng.gen();
      correctness_test!(MCByte, num);
    }
  }

}