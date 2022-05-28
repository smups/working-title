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

use std::{
  thread,
  net::{TcpStream, SocketAddr},
  time::Instant,
};

use srvr_sysproto::{
  raw_packet::RawPacketReader,
};

use crate::{
  task::Task::{self, *},
  client::{net::PackageHandler, task::TaskHandler},
  wire::Wire
};

//Module structure
mod net;
mod task;
pub mod state;

#[derive(Debug)]
pub struct Client {
  addr: SocketAddr
}

impl Client {

  pub fn new(mut stream: TcpStream, addr: SocketAddr, global_wire: &mut Wire<Task>)
    -> Self
  {
    println!("New client connected @{addr:?}");
    //Setup connection to main thread
    let global_wire_connection = global_wire.connect();
    let server_instruction_wire: Wire<Task> = Wire::new();

    thread::Builder::new().name(format!("TL_thread_@{addr:?}")).spawn(move || {

      //Task list - is emptied each tick loop
      let mut task_list: Vec<Task> = Vec::new();

      //Connections to the main server thread
      let mut listen_global = global_wire_connection;

      //Thread-local data

      //Timing states
      let mut last_tick = Instant::now();

      //Flag to differentiate between login, handshake and play states
      use state::PlayState::*;
      let mut state = HandShake;

      'tick_loop: loop {
        //(1) Listen for task from server
        task_list.push(listen_global.poll().unwrap_or(DoNothing));

        //(2) Execute tasks from previous tick
        let mut follow_up = Vec::new();
        for task in task_list.drain(..) {
          match task {
            DoNothing => {}, //do nothing lol
            Die => break 'tick_loop,
            SetServerState(new_state) => state = new_state,
            SpawnPlayer(ctx) => task::spawn_player::Handler::handle_task(ctx, &mut stream, &mut follow_up),
            SendSpawnLoc(ctx) => task::set_spawn_loc::Handler::handle_task(ctx, &mut stream, &mut follow_up),
            _ => {} //do nothing
          }
        }
        follow_up.drain(..).for_each(|task| task_list.push(task));

        //(3) Get input from the remote client (this blocks!!!)
        let mut package = RawPacketReader::read(&mut stream).unwrap();

        //(4) Find out what kind of packet we are dealing with
        let mut client_tasks = match state {
          HandShake => { match package.get_package_id() {
            0x00 => net::x00_handshake::Handler::handle_package(package, &mut stream),
            0x01 => net::x01_pingpong::Handler::handle_package(package, &mut stream),
            0xfe => net::xfe_serverlist_ping::Handler::handle_package(package, &mut stream),
            _ => vec![DoNothing]
          }},
          Login => { match package.get_package_id() {
            0x00 => net::x00_login::Handler::handle_package(package, &mut stream),
            usize::MAX => vec![Die], //client disconnected
            _ => vec![DoNothing]
          }},
          Play => { match package.get_package_id() {
            usize::MAX => vec![Die], //client disconnected
            _ => vec![DoNothing]
          }}
        };
        task_list.append(&mut client_tasks);

        /* (5)
          To prevent overloading the server we must wait if this tick-loop was
          particularly quick.

          IGNORE this if we are not in the play state!
        */
        if state != Play {continue;}
        if last_tick.elapsed() < crate::TICK_DURATION {
          thread::sleep(crate::TICK_DURATION - last_tick.elapsed());
        }
        last_tick = Instant::now(); //update last tick
      }

      //Tick-loop ended, client is dead
      println!("Client disconnected, ending tickoop {}",thread::current().name().unwrap());
    }).unwrap();

    Client { addr: addr }
  }

}