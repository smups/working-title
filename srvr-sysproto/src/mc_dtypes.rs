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

use std::{
  error::Error,
  fmt::{Display, Formatter}
};

use crate::raw_packet::{RawPacketReader, RawPacketWriter};

/*
  List of all the data types used in the MineCraft protocol
*/
//(A) fixed length numeric types
mod mc_bool;
mod mc_byte;
mod mc_ubyte;
mod mc_short;
mod mc_ushort;
mod mc_int;
mod mc_long;
mod mc_float;
mod mc_double;

//(B) variable-length numeric types
mod mc_array;
mod mc_varint;
mod mc_varlong;

//(C) Text
mod mc_string;
mod mc_chat;

//(D) Entity and world data
mod mc_uuid;
mod mc_position;
mod mc_nbt;

pub trait MCDataType {
  fn decode(buf: &mut RawPacketReader) -> Result<Self, Err> where Self: Sized;
  fn encode(&self, buf: &mut RawPacketWriter);
}

#[derive(Debug)]
pub struct MCDataTypeDecodeError(String);
type Err = MCDataTypeDecodeError;

impl Display for MCDataTypeDecodeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while decoding MCDataType: '{}'", self.0)
  }
}

impl Error for MCDataTypeDecodeError{}

/*
  Last step is to re-export all MC datatypes under this namespace
*/
//(A) fixed length numeric types
pub use mc_bool::MCBool as MCBool;
pub use mc_byte::MCByte as MCByte;
pub use mc_ubyte::MCUByte as MCUByte;
pub use mc_short::MCShort as MCShort;
pub use mc_ushort::MCUShort as MCUShort;
pub use mc_int::MCInt as MCInt;
pub use mc_long::MCLong as MCLong;
pub use mc_float::MCFloat as MCFloat;
pub use mc_double::MCDouble as MCDouble;

//(B) variable-length numeric types
pub use mc_array::MCArray as MCArray;
pub use mc_varint::MCVarInt as MCVarInt;
pub use mc_varlong::MCVarLong as MCVarLong;

//(C) Text
pub use mc_string::MCString as MCString;
pub use mc_chat::MCChat as MCChat;

//(D) Entity and world data
pub use mc_uuid::MCUuid as MCUuid;
pub use mc_position::MCPosition as MCPosition;
pub use mc_nbt::NbtTag as MCNbt;

/*
  Useful macros for testing
*/

#[macro_export]
macro_rules! correctness_test {
  ($mc_dtype:ty, $data:expr) => {
    use crate::{
      raw_packet::{RawPacketReader, RawPacketWriter},
      mc_dtypes::MCDataType
    };

    //Encode the type
    let before_io = <$mc_dtype>::from(($data));
    let mut buf = RawPacketWriter::new(0);
    before_io.encode(&mut buf);

    //Decode the type
    let mut tmp_read = RawPacketReader::from_raw(buf.to_raw());
    let after_io = <$mc_dtype>::decode(&mut tmp_read).unwrap();
    assert_eq!(before_io, after_io);
  };
}