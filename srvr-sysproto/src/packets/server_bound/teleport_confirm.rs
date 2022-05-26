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
  mc_dtypes::{MCDataType, MCVarInt}
};

#[derive(Debug, Clone)]
pub struct TeleportConfirmPacket {
  teleport_id: usize
}

impl Packet for TeleportConfirmPacket {
  const PACKET_ID: usize = 0x00;

  fn decode(buf: &mut RawPacketReader)
    -> Result<TeleportConfirmPacket, Box<dyn Error>>
  {
    Ok(TeleportConfirmPacket{
      teleport_id: i32::from(MCVarInt::decode(buf)?) as usize
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCVarInt::from(self.teleport_id as i32).encode(buf);
  }
}