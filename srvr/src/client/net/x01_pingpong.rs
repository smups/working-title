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

use super::PackageHandler;

use crate::task::Task;

use srvr_sysproto::{
  packets::{Packet, SB_Ping, CB_Pong},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream) -> Task {
    //(1) Decode ping packet
    let ping = SB_Ping::decode(&mut raw_pck).unwrap();
    println!("{ping:?}");

    //(2) Return pong packet
    let pong = CB_Pong{payload: ping.payload};
    println!("{pong:?}");
    let mut writer = RawPacketWriter::new(pong.packet_id());
    pong.encode(&mut writer);
    writer.write(stream).unwrap();

    //(3) Kill the connection
    Task::Die
  }
}