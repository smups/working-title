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

use std::string::FromUtf8Error;

use super::{*, mc_varint::MCVarInt};

#[derive(Debug, Clone)]
pub struct MCString {
  /*
    Memory layout:
    -varint (length)
    -utf8 string
  */
  len: usize,
  txt: String
}

impl MCDataType for MCString {
  
  fn decode(buf: &mut RawPacketReader) -> Result<MCString, Err> {
    //Get length of string (this automatically advances the RawPacketReader)
    let len: i32 = MCVarInt::decode(buf)?.into();

    //Set-up a byte vector
    let mut internal_buf = vec![0x00u8; len as usize];
    buf.read_into(&mut internal_buf);

    Ok(MCString{len: len as usize, txt: String::from_utf8(internal_buf)?})
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    //(1) Encode the length in the writer
    MCVarInt::from(self.len as i32).encode(buf);

    //(2) Encode the string itself
    buf.write_bytes(self.txt.as_bytes());
  }
}

impl MCString {
  pub fn new(txt: String) -> MCString {
    MCString{ len: txt.len(), txt: txt }
  }
}

impl From<MCString> for String {
  fn from(a: MCString) -> String {a.txt}
}

impl From<String> for MCString {
  fn from(a: String) -> MCString {MCString{len: a.len(), txt: a }}
}

impl From<FromUtf8Error> for MCDataTypeDecodeError {
  fn from(err: FromUtf8Error) -> Self {MCDataTypeDecodeError(format!("{err}"))}
}