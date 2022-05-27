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

use crate::task::{Task, SpawnPlayerCtx, SpawnLocCtx};
use super::TaskHandler;

#[derive(Debug)]
pub struct Handler;

const SPAWN_LOC: (i32, i32, i16) = (0,0,100);
const SPAWN_ANG: f32 = 120.;

impl TaskHandler for Handler {
  type Context = SpawnPlayerCtx;
  fn handle_task(task: SpawnPlayerCtx, stream: &mut TcpStream, task_list: &mut Vec<Task>) {
    use Task::*;

    //(1) Send spawn position
    let pos_ctx = SpawnLocCtx{location: SPAWN_LOC, angle: SPAWN_ANG};
    task_list.push(SendSpawnLoc(pos_ctx));
  }
}