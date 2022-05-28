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
  mc_dtypes::{MCDataType, MCPosition, MCFloat}
};

#[derive(Debug,Clone)]
pub struct SpawnPositionPacket {
  pub location: (i32, i32, i16),
  pub angle: f32
}

impl Packet for SpawnPositionPacket {
  const PACKET_ID: usize = 0x4b;

  fn decode(buf: &mut RawPacketReader)
    -> Result<SpawnPositionPacket, Box<dyn Error>>
  {
    let position = MCPosition::decode(buf)?;
    let angle = MCFloat::decode(buf)?;
    Ok(SpawnPositionPacket{
      location: position.into(),
      angle: angle.into()
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCPosition::from(self.location).encode(buf);
    MCFloat::from(self.angle).encode(buf);
  }

}