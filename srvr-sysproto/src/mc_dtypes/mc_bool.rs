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