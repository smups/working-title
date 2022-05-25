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

use rand::Rng;

use super::PackageHandler;

use crate::{
  client::state::PlayState::*,
  task::Task,
};

use srvr_sysproto::{
  packets::{SB_LoginStart, Packet, CB_LoginSuccess},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream) -> Task {
    //(1) Decode the package
    let login_req = SB_LoginStart::decode(&mut raw_pck).unwrap();
    println!("{login_req:?}");

    //For now, we'll forgo encryption. Just get the username and think of an UUID
    let username = login_req.player_name;
    let uuid = rand::thread_rng().gen::<u128>();

    //(2) Reply with a Login Success packet
    let rsp = CB_LoginSuccess{uuid: uuid, player_name: username};
    let mut writer = RawPacketWriter::new(rsp.packet_id());
    rsp.encode(&mut writer);
    writer.write(stream).unwrap();

    //(3) Give command to change server state to play
    Task::SetServerState(Play)
  }
}