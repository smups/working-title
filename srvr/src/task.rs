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

use srvr_sysproto::mc_dtypes::MCPosition;

use crate::client::state::PlayState;

#[derive(Debug, Clone)]
pub enum Task {
  /*
    Tasks are used to communicate both within a thread (to relay data or
    instructions to the next tick loop) or between threads (to relay
    instructions and basic data).

    Tasks may contain a small data payload (context), which is heap-allocated
    and passed via a box.
    Task context types implement the TaskContext trait.
  */
  DoNothing,
  Die,
  SetServerState(PlayState),
  //Player spawn process
  SpawnPlayer(SpawnPlayerCtx),
  SendSpawnLoc(SpawnLocCtx)
}

//See documentation above
pub trait TaskContext {}

#[derive(Debug, Clone)]
pub struct SpawnPlayerCtx {
  pub player_name: String,
  pub uuid: u128
}
impl TaskContext for SpawnPlayerCtx {}

#[derive(Debug, Clone)]
pub struct SpawnLocCtx {
  pub location: (i32, i32, i16),
  pub angle: f32
}
impl TaskContext for SpawnLocCtx {}