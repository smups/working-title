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


use std::net::TcpStream;

use srvr_sysproto::{
  packets::{Packet, CB_SpawnPosition},
  raw_packet::RawPacketWriter
};

use crate::task::{Task, SpawnLocCtx};
use super::TaskHandler;

#[derive(Debug)]
pub struct Handler;

impl TaskHandler for Handler {
  type Context = SpawnLocCtx;
  fn handle_task(ctx: SpawnLocCtx, stream: &mut TcpStream, _: &mut Vec<Task>) {
    //Send position
    let mut writer = RawPacketWriter::new(CB_SpawnPosition::PACKET_ID);
    CB_SpawnPosition {
      location: ctx.location,
      angle: ctx.angle
    }.encode(&mut writer);
    writer.write(stream).unwrap();
  }
}