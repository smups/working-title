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
  packets::{SB_HandshakePacket, Packet, CB_StatusPacket},
  raw_packet::{RawPacketReader, RawPacketWriter}
};

#[derive(Debug)]
pub struct Handler;

impl PackageHandler for Handler {
  fn handle_package(mut raw_pck: RawPacketReader, stream: &mut TcpStream) -> Task {
    //(1) Decode handshake
    let handshake = SB_HandshakePacket::decode(&mut raw_pck).unwrap();
    println!("{handshake:?}");

    //(2) Decide where we go next
    let next_step = match handshake.next_state_code() {
      0x01 => {
        //Code 1: Status Request, for now, we send a basic JSON response:

        //(2.1) Create sample response
        let response = CB_StatusPacket::new(format!("{{
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
        
        //(2.2) Reply with response
        let mut writer = RawPacketWriter::new(200);
        response.encode(&mut writer);
        writer.write(stream).unwrap();

        Task::DoNothing
      } 0x02 => {
        //Code 2: Login Request
        todo!();
      } other => Task::DoNothing
    };

    return next_step;
  }
}