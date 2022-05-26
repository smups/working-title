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
  mc_dtypes::{MCDataType, MCNbt, MCInt, MCBool, MCUByte, MCVarInt, MCArray, MCString, MCLong}
};

#[derive(Debug,Clone)]
pub struct JoinGamePacket {
  entity_id: u32,
  hardcore: bool,
  gamemode: u8,
  prev_gamemode: u8,
  world_count: usize,
  world_names: Vec<String>,
  world_codecs: MCNbt,
  spawn_world_codec: MCNbt,
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

impl Packet for JoinGamePacket {
  const PACKET_ID: usize = 0x26;

  fn decode(buf: &mut RawPacketReader) -> Result<JoinGamePacket, Box<dyn Error>> {

    //First batch of simple types
    let entity_id: i32 = MCInt::decode(buf)?.into();
    let hardcore: bool = MCBool::decode(buf)?.into();
    let gamemode: u8 = MCUByte::decode(buf)?.into();
    let prev_gamemode: u8 = MCUByte::decode(buf)?.into();
    let world_count: i32 = MCVarInt::decode(buf)?.into();

    //Array requires special attention
    let world_names = Vec::from(MCArray::<MCString>::decode(world_count as usize, buf)?)
      .into_iter()
      .map(|mc_string| String::from(mc_string))
      .collect::<Vec<String>>();

    //Now we get some large NBT's
    let world_codecs = MCNbt::decode(buf)?;
    let spawn_world_codec = MCNbt::decode(buf)?;
    
    //And another bunch of basic types
    let spawn_world_name: String = MCString::decode(buf)?.into();
    let seed: i64 = MCLong::decode(buf)?.into();
    let max_players: i32 = MCVarInt::decode(buf)?.into();
    let view_distance: i32 = MCVarInt::decode(buf)?.into();
    let sim_distance: i32 = MCVarInt::decode(buf)?.into();
    let reduced_debug_info: bool = MCBool::decode(buf)?.into();
    let enable_respawn_screen: bool = MCBool::decode(buf)?.into();
    let debug: bool = MCBool::decode(buf)?.into();
    let flat: bool = MCBool::decode(buf)?.into();

    Ok(JoinGamePacket{
      entity_id: entity_id as u32,
      hardcore: hardcore,
      gamemode: gamemode,
      prev_gamemode: prev_gamemode,
      world_count: world_count as usize,
      world_names: world_names,
      world_codecs: world_codecs,
      spawn_world_codec: spawn_world_codec,
      spawn_world_name: spawn_world_name,
      seed: seed as u64,
      max_players: max_players as usize,
      view_distance: view_distance as usize,
      sim_distance: sim_distance as usize,
      reduced_debug_info: reduced_debug_info,
      enable_respawn_screen: enable_respawn_screen,
      debug: debug,
      flat: flat
    })
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
      todo!()
  }
}