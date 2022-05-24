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
  mc_dtypes::{MCDataType, MCUuid, MCString}
};

#[derive(Debug,Clone)]
pub struct LoginSuccessPacket {
  pub uuid: u128,
  pub player_name: String
}

impl Packet for LoginSuccessPacket {

  const PACKET_ID: usize = 0x02;

  fn decode(buf: &mut RawPacketReader) -> Result<LoginSuccessPacket, Box<dyn Error>> {
    let uuid: u128 = MCUuid::decode(buf)?.into();
    let player_name: String = MCString::decode(buf)?.into();
    Ok(LoginSuccessPacket{ uuid: uuid, player_name: player_name})
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    MCUuid::from(self.uuid).encode(buf);
    MCString::from(self.player_name.clone()).encode(buf);
  }

}