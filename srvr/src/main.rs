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

use std::{
  error::Error,
  net::TcpListener, time::Duration,
};

pub mod state;
pub mod client;
pub mod task;
pub mod wire;

use libloading::*;
use srvr_sysplugin::Plugin;

use crate::{
  client::Client,
  wire::Wire,
  task::Task
};

//Tick duration
pub const TICK_DURATION: Duration = Duration::from_millis(50);
const READ_TIMEOUT: Duration = Duration::from_millis(5);

fn main() -> Result<(), Box<dyn Error>> {
  println!("Starting Server!");

  //(1) Load plugins
  let lib = unsafe {
  Library::new("target/debug/libsample_plugin.so")
  }.unwrap();

  let linker: Symbol<extern "Rust" fn() -> Box<dyn Plugin>> = unsafe {
  lib.get(b"link")
  }.unwrap();

  let mut plugin = linker();
  plugin.as_mut().start();

  /*
    Global resources
  */
  //Global wire to all clients
  let mut global_wire: Wire<Task> = Wire::new();

  //Wires to all clients
  let mut client_wires: Vec<Client> = Vec::new();

  //(2) Connect to port
  let socket = TcpListener::bind("127.0.0.1:25565")?;

  loop {
    // (1) Open connection
    let (stream, addr) = socket.accept()?;
    stream.set_read_timeout(Some(READ_TIMEOUT)).unwrap();
    let client = Client::new(stream, addr, &mut global_wire);
    client_wires.push(client);
  }

  //Drop clients before we exit
  drop(client_wires);

  plugin.as_mut().stop();
}