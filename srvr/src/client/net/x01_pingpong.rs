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

use super::PackageHandler;

use crate::task::Task;

use srvr_sysproto::{
  packets::{Packet, SB_Ping, CB_Pong},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream)
    -> Vec<Task>
  {
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
    vec![Task::Die]
  }
}