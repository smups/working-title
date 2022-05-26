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

use byteorder::{BigEndian, ByteOrder};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCUuid(u128);

impl MCDataType for MCUuid {

  fn decode(buf: &mut RawPacketReader) -> Result<MCUuid, Err> {
    Ok(MCUuid(BigEndian::read_u128(&buf.read_bytes(16))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&u128::to_be_bytes((*self).into()))
  }
}

impl From<u128> for MCUuid{
  fn from(val: u128) -> Self {MCUuid(val)}
}

impl From<MCUuid> for u128 {
  fn from(val: MCUuid) -> Self {val.0}
}

#[cfg(test)]
mod mc_double_test {

  use rand::{self, Rng};

  use crate::correctness_test;

  #[test]
  fn correctness_test() {
    let mut rng = rand::thread_rng();
    //Try 100 random bytes
    for _ in 0..100 {
      let num: u128 = rng.gen();
      correctness_test!(crate::mc_dtypes::MCUuid, num);
    }
  }

}