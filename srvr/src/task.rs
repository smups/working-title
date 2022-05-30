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

use crate::client::state::PlayState;

#[derive(Clone)]
pub enum Task {
  /*
    Tasks are used to communicate both within a thread (to relay data or
    instructions to the next tick loop) or between threads (to relay
    instructions and basic data).

    The generic task Do(function_ptr) executes the function the function pointer
    is pointing to, provided the context (enum). Generic tasks can only directly
    modify two aspects of the tick loop:
      (1) Add follow-up tasks (this is done via the follow_up borrow)
      (2) Send client-bound packages (done via the stream borrow)
    If more control over the tick-loop is required, a task gets a separate enum
    variant. This amount should be kept to a minimum to prevent unnecessary
    branching.
  */
  DoNothing,
  Die,
  Do(
    fn(ctx: TaskContext, stream: &mut TcpStream, follow_up: &mut Vec<Task>)
      -> Result<(), Box<dyn std::error::Error>>,
    TaskContext//<'a> //The context is packaged together with the function pointer
  ),
  SetServerState(PlayState)
}

impl std::fmt::Debug for Task {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::DoNothing => write!(f, "DoNothing"),
      Self::Die => write!(f, "Die"),
      Self::Do(_exe, ctx) => write!(f, "Do({ctx:?})"),
      Self::SetServerState(arg0) => f.debug_tuple("SetServerState").field(arg0).finish(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum TaskContext {//<'a> {
  /*
    This enum contains data (context) that is required to perform a certain
    task. If a large amount of data is required to perform the task, a reference
    or smart pointer should be used.
  */
  SpawnPlayerCtx{player_name: String, uuid: u128},
  SpawnLocCtx{location: (i32, i32, i16), angle: f32}
}