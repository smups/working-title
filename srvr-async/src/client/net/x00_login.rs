/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 15
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use std::net::TcpStream;

use rand::Rng;

use super::PackageHandler;

use crate::{
  client::{state::PlayState::*, task_executors},
  task::{Task, TaskContext::*}
};

use srvr_sysproto::{
  packets::{SB_LoginStart, Packet, CB_LoginSuccess},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream)
    -> Vec<Task>
  {
    //(1) Decode the package
    let login_req = SB_LoginStart::decode(&mut raw_pck).unwrap();

    //For now, we'll forgo encryption. Just get the username and think of an UUID
    let username = login_req.player_name;
    let uuid = rand::thread_rng().gen::<u128>();

    //(2) Reply with a Login Success packet
    let rsp = CB_LoginSuccess{uuid: uuid.clone(), player_name: username.clone()};
    let mut writer = RawPacketWriter::new(rsp.packet_id());
    rsp.encode(&mut writer);
    writer.write(stream).unwrap();

    //(3) Prepare context for spawn player command
    let ctx = SpawnPlayerCtx{ player_name: username, uuid: uuid };
    let executor = task_executors::spawn_player;

    //(4) Send commands
    vec![
      Task::SetServerState(Play),
      Task::Do(executor, ctx)
    ]
  }
}