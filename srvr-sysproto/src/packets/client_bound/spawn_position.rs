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