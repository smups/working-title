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
  mc_dtypes::{MCDataType, MCLong}
};

#[derive(Debug,Clone)]
pub struct JoinGamePacket {
  entity_id: u32,
  hardcore: bool,
  gamemode: u8,
  prev_gamemode: u8,
  world_count: usize,
  world_names: Vec<String>,
  //world_codecs:,
  //spawn_world_codec,
  spawn_world_name: String,
  seed: u64,
  max_players: usize,
  view_distance: usize,
  sim_distance: usize,
  reduced_debug_info: bool,
  enable_respawn_screen: bool,
  debug: bool,
  flat: bool
}