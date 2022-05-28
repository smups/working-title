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

use srvr_sysproto::raw_packet::RawPacketReader;

use crate::task::Task;

pub trait PackageHandler {
  fn handle_package(raw_pck: RawPacketReader, stream: &mut TcpStream) -> Vec<Task>;
}

/*
  List of package handlers
*/
//(A) handshake procedure
pub mod x00_handshake;
pub mod x01_pingpong;
pub mod xfe_serverlist_ping;

//(B) login procedure
pub mod x00_login;

//(C) Play