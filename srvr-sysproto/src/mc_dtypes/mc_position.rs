/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 17
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

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCPosition(i32, i32, i16);

impl MCDataType for MCPosition {

  fn decode(buf: &mut RawPacketReader) -> Result<MCPosition, Err> {
    /*  Reminder on the layout of this uint:
      -The 26 MOST significant bits are the x-component (24b signed int)
      -The 26 MIDDLE bits are the z-component (24b signed int)
      -The 12 LEAST significant bits are the y-component (12b signed int)
    */
    let raw = BigEndian::read_u64(&buf.read_bytes(8));

    let mut x = ((raw >> 38) as u32) & 0x3FFFFFF;
    let mut z = ((raw >> 12) & 0x3FFFFFF) as u32;
    let mut y = ((raw & 0xFFF) as u16) & 0xFFF;

    if (x & 0x2000000) != 0 {
      // is the 26th bit set
      // if so, treat the rest as a positive integer, and treat 26th bit as -2^25
      // 2^25 == 0x2000000
      // 0x1FFFFFF == 2^26 - 1 (all places set to 1 except 26th place)
      x = (((x & 0x1FFFFFF) as i32) - 0x2000000) as u32;
    }
    if (y & 0x800) != 0 {
      y = (((y & 0x7FF) as i16) - 0x800) as u16;
    }
    if (z & 0x2000000) != 0 {
      z = (((z & 0x1FFFFFF) as i32) - 0x2000000) as u32;
    }

    Ok(MCPosition(x as i32, z as i32, y as i16))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {

    let x_raw = if self.0 < 0 {
      (self.0 + 0x2000000) as u64 | 0x2000000
    } else {
      self.0 as u64
    } & 0x3FFFFFF;

    let z_raw = if self.1 < 0 {
      (self.1 + 0x2000000) as u64 | 0x2000000
    } else {
      self.1 as u64
    } & 0x3FFFFFF;

    let y_raw = if self.2 < 0 {
      (self.2 + 0x800) as u64 | 0x800
    } else {
      self.2 as u64
    } & 0xFFF;

    buf.write_bytes(&((x_raw << 38) | (z_raw << 12) | y_raw).to_be_bytes());
  }
}

impl From<(i32, i32, i16)> for MCPosition {
  fn from(tup: (i32, i32, i16)) -> Self {MCPosition(tup.0, tup.1, tup.2)}
}

impl From<MCPosition> for (i32, i32, i16) {
  fn from(pos: MCPosition) -> Self {(pos.0, pos.1, pos.2)}
}

#[cfg(test)]
mod mc_position_test {

  use rand::{self, Rng};

  use crate::correctness_test;

  #[test]
  fn correctness_test() {
    let mut rng = rand::thread_rng();

    //Some constants
    const I26_MIN: i32 = -33554432;
    const I26_MAX: i32 = 33554431;
    const I12_MIN: i16 = -2048;
    const I12_MAX: i16 = 2047;

    for _ in 0..100 {
      let x: i32 = rng.gen_range(I26_MIN..I26_MAX);
      let y: i16 = rng.gen_range(I12_MIN..I12_MAX);
      let z: i32 = rng.gen_range(I26_MIN..I26_MAX);
      let pos = (x,z,y);
      correctness_test!(crate::mc_dtypes::MCPosition, pos);
    }
  }

}