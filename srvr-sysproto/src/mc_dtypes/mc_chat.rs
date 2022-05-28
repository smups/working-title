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

use serde_json::Value;

use crate::raw_packet::{RawPacketReader, RawPacketWriter};

use super::{MCDataType, MCString, MCDataTypeDecodeError};

#[derive(Debug, Clone)]
pub struct MCChat {
  chat: Value
}

impl MCDataType for MCChat {
  
  fn decode(buf: &mut RawPacketReader) -> Result<MCChat, MCDataTypeDecodeError> {
    //(1) Get the underlying MCString
    let string: String = MCString::decode(buf)?.into();
    
    //(2) Parse the string with serde to a JSON object
    let json: Value = serde_json::from_str(&string)?;

    Ok(MCChat{chat: json})
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    //(1) Convect JSON to string
    let string = serde_json::to_string(&self.chat).unwrap();
    
    //(2) Encode string as MCString
    MCString::new(string).encode(buf);
  }
}

impl MCChat {

  pub fn from_string(chat: &str) -> Result<MCChat, serde_json::Error> {
    Ok(MCChat{ chat: serde_json::from_str(chat)? })
  }

}

impl From<MCChat> for Value {
  fn from(a: MCChat) -> Value {a.chat}
}

impl From<Value> for MCChat {
  fn from(a: Value) -> MCChat {MCChat{ chat: a }}
}

impl From<serde_json::Error> for MCDataTypeDecodeError {
  fn from(err: serde_json::Error) -> MCDataTypeDecodeError {
    MCDataTypeDecodeError(format!("{err}"))
  }
}