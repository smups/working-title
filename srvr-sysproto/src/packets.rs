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
  raw_packet::{RawPacketReader, RawPacketWriter}
};

//Module structure
mod server_bound;
mod client_bound;

pub trait Packet {
  const PACKET_ID: usize;

  fn decode(buf: &mut RawPacketReader) -> Result<Self, Box<dyn Error>> where Self: Sized;
  fn encode(&self, buf: &mut RawPacketWriter);
  fn packet_id(&self) -> usize {Self::PACKET_ID}
}

/*
  Re-export of all Possible server-bound (incoming) packages
*/
//(A) Handshake procedure
pub use server_bound::handshake::HandshakePacket as SB_HandshakePacket;
pub use server_bound::ping::PingPacket as SB_PingPacket;

/*
  Re-export of all Possible client-bound (outgoing) packages
*/
//(A) Handshake procedure
pub use client_bound::status::StatusPacket as CB_StatusPacket;
pub use client_bound::pong::PongPacket as CB_PongPacket;