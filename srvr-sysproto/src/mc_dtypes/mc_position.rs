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

#[cfg(test)]
mod mc_position_test {

  use crate::{
    mc_dtypes::{MCDataType, mc_position::MCPosition},
    raw_packet::{RawPacketReader, RawPacketWriter}
  };

  macro_rules! io_correctness_test {
      ($x:expr, $y:expr, $z:expr) => {
        let test_pos = MCPosition(($x), ($y), ($z));
        let mut test_write_buf = RawPacketWriter::from_raw(vec![0u8;8]);
        test_pos.encode(&mut test_write_buf);

        let mut test_read_buf = RawPacketReader::from_raw(test_write_buf.to_raw());
        let rw_test_pos = MCPosition::decode(&mut test_read_buf).unwrap();
        assert_eq!(test_pos, rw_test_pos);
      };
  }

  #[test]
  fn io_correctness_test() {
    for x in [-33554432, 0, 33554431] {
      for z in [-33554432, 0, 33554431] {
        for y in [-2048, 0, 2047] {
          io_correctness_test!(x, z, y);
        }
      }
    }
  }

}