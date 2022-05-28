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

use std::error::Error;

use crate::{
  packets::Packet,
  raw_packet::{RawPacketReader, RawPacketWriter},
  mc_dtypes::{MCDataType, MCVarInt}
};

#[derive(Debug,Clone)]
pub struct EncryptionResponsePacket {
  secret_key: Vec<u8>,
  token: Vec<u8>
}

impl Packet for EncryptionResponsePacket {

  const PACKET_ID: usize = 0x01;

  fn decode(buf: &mut RawPacketReader)
    -> Result<EncryptionResponsePacket, Box<dyn Error>>
  {
    let secret_len: i32 = MCVarInt::decode(buf)?.into();
    let secret_key = buf.read_bytes(secret_len as usize);
    let token_len: i32 = MCVarInt::decode(buf)?.into();
    let token = buf.read_bytes(token_len as usize);

    Ok(EncryptionResponsePacket {
      secret_key: secret_key, token: token
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    //(1) Get token and key length
    let token_len = self.token.len();
    let key_len = self.secret_key.len();

    //(2) Write in correct order
    MCVarInt::from(key_len as i32).encode(buf);
    buf.write_bytes(&self.secret_key);
    MCVarInt::from(token_len as i32).encode(buf);
    buf.write_bytes(&self.token);
  }

}