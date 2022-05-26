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

use std::{
  sync::mpsc::{channel, Sender, Receiver},
  thread,
  net::{TcpStream, SocketAddr},
  time::{Instant, Duration},
};

use srvr_sysproto::{
  raw_packet::RawPacketReader,
};

use crate::{
  task::Task::{self, *},
  client::net::PackageHandler
};

//Module structure
mod net;
pub mod state;

#[derive(Debug)]
pub struct Client {
  addr: SocketAddr,
  tx: Sender::<Vec<Task>>,
  rx: Receiver::<Vec<Task>>
}

impl Client {

  pub fn new(mut stream: TcpStream, addr: SocketAddr) -> Self {
    println!("New client connected @{addr:?}");
    let (tx, rx) = channel();

    thread::Builder::new().name(format!("TL_thread_@{addr:?}")).spawn(move || {
      //Task list - is emptied each tick loop
      let mut task_list: Vec<Task> = Vec::new();

      //Timing states
      let mut last_tick = Instant::now();

      //Flag to differentiate between login, handshake and play states
      use state::PlayState::*;
      let mut state = HandShake;

      'tick_loop: loop {
        //(1) Get input from the remote client
        let mut package = RawPacketReader::read(&mut stream).unwrap();
        println!("{package:?}");

        //(2) Find out what kind of packet we are dealing with
        let client_task = match state {
          HandShake => { match package.get_package_id() {
            0x00 => net::x00_handshake::Handler::handle_package(package, &mut stream),
            0x01 => net::x01_pingpong::Handler::handle_package(package, &mut stream),
            0xfe => net::xfe_serverlist_ping::Handler::handle_package(package, &mut stream),
            _ => DoNothing
          }},
          Login => { match package.get_package_id() {
            0x00 => net::x00_login::Handler::handle_package(package, &mut stream),
            _ => DoNothing
          }},
          Play => {
            //Not implemented yet
            DoNothing
          }
        };
        task_list.push(client_task);

        //(3) Execute tasks
        for task in task_list.drain(..) {
          match task {
            DoNothing => {}, //do nothing lol
            Die => break 'tick_loop,
            SetServerState(new_state) => state = new_state
          }
        }

        /* (4)
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

    Client { addr: addr, tx: tx, rx: rx }
  }

}