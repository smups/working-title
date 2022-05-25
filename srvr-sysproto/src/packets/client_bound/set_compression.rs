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
  mc_dtypes::{MCDataType, MCLong, MCVarInt}
};

#[derive(Debug,Clone)]
pub struct SetCompressionPacket {
  pub threshold_len: usize
}

impl Packet for SetCompressionPacket {

  const PACKET_ID: usize = 0x03;

  fn decode(buf: &mut RawPacketReader)
    -> Result<SetCompressionPacket, Box<dyn Error>>
  {
    Ok(SetCompressionPacket{
      threshold_len: i32::from(MCVarInt::decode(buf)?) as usize
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCVarInt::from(self.threshold_len as i32).encode(buf);
  }

}