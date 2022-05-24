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

use srvr_sysproto::raw_packet::RawPacketReader;

use crate::task::Task;

pub trait PackageHandler {
  fn handle_package(raw_pck: RawPacketReader, stream: &mut TcpStream) -> Task;
}

/*
  List of package handlers
*/
//(A) handshake procedure
pub mod x00_handshake;
pub mod x01_pingpong;