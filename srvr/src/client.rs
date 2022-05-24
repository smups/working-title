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
  net::{TcpStream, SocketAddr}, fmt::format
};

use srvr_sysproto::{
  raw_packet::RawPacketReader,
  packets::{SB_HandshakePacket, Packet}
};

use crate::{
  task::Task::{self, *},
  client::net::PackageHandler
};

mod net;

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
      let mut task_list: Vec<Task> = Vec::new();

      'tick_loop: loop {
        //(1) Get input from the remote client
        let mut package = RawPacketReader::read(&mut stream).unwrap();

        //(2) Find out what kind of packet we are dealing with
        let client_tasks = match package.get_package_id() {
          0x00 => net::x00_handshake::Handler::handle_package(package, &mut stream),
          0x01 => net::x01_pingpong::Handler::handle_package(package, &mut stream),
          _ => {DoNothing} //Request not implemented
        };
        task_list.push(client_tasks);

        //(3) Execute tasks
        for task in task_list.drain(..) {
          match task {
            DoNothing => {}, //do nothing lol
            Die => break 'tick_loop
          }
        }
      }

      //Tick-loop ended, client is dead
      println!("Client disconnected, ending tickoop {}",thread::current().name().unwrap());
    }).unwrap();

    Client { addr: addr, tx: tx, rx: rx }
  }

}