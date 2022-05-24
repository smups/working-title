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
pub struct MCBool(bool);

impl MCDataType for MCBool {

  fn decode(buf: &mut RawPacketReader) -> Result<MCBool, Err> {
    match buf.read_byte() {
      0x00 => Ok(MCBool(false)),
      0x01 => Ok(MCBool(true)),
      _ => Err(MCDataTypeDecodeError("incorrect boolean encountered".to_string()))
    }
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    match (*self).into() {
      false => buf.write_byte(0x00),
      true => buf.write_byte(0x01)
    }
  }
}

impl From<bool> for MCBool{
  fn from(val: bool) -> Self {MCBool(val)}
}

impl From<MCBool> for bool {
  fn from(val: MCBool) -> Self {val.0}
}

#[cfg(test)]
mod mc_bool_test {

  use std::vec;

use crate::{
    mc_dtypes::{MCDataType, mc_bool::MCBool},
    raw_packet::{RawPacketReader, RawPacketWriter}
  };

  macro_rules! read_test {
    ($bytes:expr, $num:expr) => {
      assert_eq!(
        MCBool::decode(
          &mut RawPacketReader::from_raw(($bytes))).unwrap(),
          MCBool(($num)
        )
      )
    };
  }

  macro_rules! write_test {
      ($bytes:expr, $num:expr) => {
          let mut buf = RawPacketWriter::new(0);
          MCBool::from(($num)).encode(&mut buf);
          assert_eq!(&($bytes), buf.raw_view());
      };
  }

  #[test]
  fn read_test() {
    read_test!(vec![0x00], false);
    read_test!(vec![0x01], true);
  }

  #[test]
  fn write_test() {
    write_test!(vec![0x00], false);
    write_test!(vec![0x01], true);
  }

}