/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 15
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use byteorder::{BigEndian, ByteOrder};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCShort(i16);

impl MCDataType for MCShort {

  fn decode(buf: &mut RawPacketReader) -> Result<MCShort, Err> {
    Ok(MCShort(BigEndian::read_i16(&buf.read_bytes(2))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&i16::to_be_bytes((*self).into()))
  }
}

impl From<i16> for MCShort{
  fn from(val: i16) -> Self {MCShort(val)}
}

impl From<MCShort> for i16 {
  fn from(val: MCShort) -> Self {val.0}
}

#[cfg(test)]
mod mc_double_test {

  use rand::{self, Rng};

  #[macro_use]
  use crate::correctness_test;

  #[test]
  fn correctness_test() {
    let mut rng = rand::thread_rng();
    //Try 100 random bytes
    for _ in 0..100 {
      let num: i16 = rng.gen();
      correctness_test!(crate::mc_dtypes::MCShort, num);
    }
  }

}