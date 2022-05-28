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