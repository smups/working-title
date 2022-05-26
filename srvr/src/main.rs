use std::{
  error::Error,
  net::TcpListener, time::Duration,
};

use libloading::*;

use srvr_sysplugin::Plugin;

use crate::client::Client;

//Tick duration
pub const TICK_DURATION: Duration = Duration::from_millis(50);

pub mod state;
pub mod client;
pub mod task;

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

  //(2) Connect to port
  let socket = TcpListener::bind("127.0.0.1:25565")?;
  loop {
    // (1) Open connection
    let (mut stream, addr) = socket.accept()?;
    let client = Client::new(stream, addr);
  }

  plugin.as_mut().stop();
}