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

  //Global resources
  //Global wire to all clients
  let mut global_wire: Wire<Task> = Wire::new();

  //(2) Connect to port
  let socket = TcpListener::bind("192.168.2.11:25565")?;
  loop {
    // (1) Open connection
    let (mut stream, addr) = socket.accept()?;
    let client = Client::new(stream, addr, &mut global_wire);
  }

  plugin.as_mut().stop();
}