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
  mc_dtypes::{MCVarInt, MCDataType, MCString, MCUShort}
};

const PACKET_ID: usize = 0x00;

#[derive(Debug,Clone)]
pub struct HandshakePacket {
  proto_ver: usize,
  server_addr: String,
  server_port: u16,
  next_state: u8
}

impl Packet for HandshakePacket {

  fn decode(buf: &mut RawPacketReader) -> Result<HandshakePacket, Box<dyn Error>> {
    let proto_ver: i32 = MCVarInt::decode(buf)?.into();
    let server_addr: String = MCString::decode(buf)?.into();
    let server_port: u16 = MCUShort::decode(buf)?.into();
    let next_state: i32 = MCVarInt::decode(buf)?.into();

    Ok(HandshakePacket{
      proto_ver: proto_ver as usize,
      server_addr: server_addr,
      server_port: server_port,
      next_state: next_state as u8
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCVarInt(self.proto_ver as i32).encode(buf);
    MCString::new(self.server_addr.clone()).encode(buf);
    MCUShort(self.server_port).encode(buf);
    MCVarInt(self.next_state as i32).encode(buf);
  }

  fn packet_id(&self) -> usize {PACKET_ID}

}

impl HandshakePacket {

  pub fn new() -> Self {todo!()}

  pub fn get_protocol(&self) -> usize {self.proto_ver}
  pub fn get_server_ip(&self) -> &str {&self.server_addr}
  pub fn get_server_port(&self) -> u16 {self.server_port}
  pub fn next_state_code(&self) -> u8 {self.next_state}

}