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
  raw_packet::{RawPacketReader, RawPacketWriter},
  mc_dtypes::{MCDataType, MCDataTypeDecodeError, MCVarInt}
};

/*
  List of all Possible packages
*/
//(A) Handshake procedure
mod handshake;

#[derive(Debug, Clone, Copy)]
pub struct PacketFormat {
  pub length: usize,
  pub packet_id: usize
}

impl MCDataType for PacketFormat {

  fn decode(buf: &mut RawPacketReader) -> Result<PacketFormat, MCDataTypeDecodeError> {
    //Both the length and packetID are varints
    let length: i32 = MCVarInt::decode(buf)?.into();
    let packet_id: i32 = MCVarInt::decode(buf)?.into();

    Ok(PacketFormat{length: length as usize, packet_id: packet_id as usize})
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCVarInt(self.length as i32).encode(buf);
    MCVarInt(self.packet_id as i32).encode(buf);
  }

}

pub trait Packet {
  fn decode(buf: &mut RawPacketReader) -> Result<Self, Box<dyn Error>> where Self: Sized;
  fn encode(&self, buf: &mut RawPacketWriter);
  fn packet_id(&self) -> usize;
}

/*
  Re-export of all Possible packages
*/
//(A) Handshake procedure
pub use handshake::HandshakePacket as HandshakePacket;