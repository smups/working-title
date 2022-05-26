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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MCDouble(f64);

impl MCDataType for MCDouble {

  fn decode(buf: &mut RawPacketReader) -> Result<MCDouble, Err> {
    Ok(MCDouble(BigEndian::read_f64(&buf.read_bytes(8))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&f64::to_be_bytes((*self).into()))
  }
}

impl From<f64> for MCDouble{
  fn from(val: f64) -> Self {MCDouble(val)}
}

impl From<MCDouble> for f64 {
  fn from(val: MCDouble) -> Self {val.0}
}

#[cfg(test)]
mod mc_double_test {

  use rand::{self, Rng};

  #[macro_use]
  use crate::correctness_test;

  #[test]
  fn correctness_test() {
    use crate::mc_dtypes::MCDouble;
    let mut rng = rand::thread_rng();
    //Try 100 random bytes
    for _ in 0..100 {
      let num: f64 = rng.gen();
      correctness_test!(MCDouble, num);
    }
  }

}