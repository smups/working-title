/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 17
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

use crate::{
  client::state::PlayState::*,
  task::Task,
};

use srvr_sysproto::{
  packets::{SB_Handshake, Packet, CB_Status},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream)
    -> Vec<Task>
  {
    //(1) Decode handshake
    let handshake = SB_Handshake::decode(&mut raw_pck).unwrap();
    println!("{handshake:?}");

    //(2) Decide where we go next
    let next_step = match handshake.next_state_code() {
      0x01 => {
        //Code 1: Status Request, for now, we send a basic JSON response:

        //(2.1) Create sample response
        let response = CB_Status::new(format!("{{
          \"version\": {{
            \"name\": \"1.18.2\",
            \"protocol\": {}
          }},
          \"players\": {{
            \"max\": 1000,
            \"online\": 5,
            \"sample\": [
              {{
                \"name\": \"thinkofdeath\",
                \"id\": \"4566e69f-c907-48ee-8d71-d7ba5aa00d20\"
              }}
            ]
          }},
          \"description\": {{
            \"text\": \"Hello world\"
          }}
        }}", srvr_sysproto::PROTOCOL_VERSION));
        println!("{response:?}");
        
        //(2.2) Reply with response
        let mut writer = RawPacketWriter::new(response.packet_id());
        response.encode(&mut writer);
        writer.write(stream).unwrap();

        //(R) Do nothing
        vec![Task::DoNothing]
      } 0x02 => {
        //Code 2: Login Request, change login flag to true
        vec![Task::SetServerState(Login)]
      } _ => vec![Task::Die]
    };

    return next_step;
  }
}