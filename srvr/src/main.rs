use std::{
  error::Error,
  net::TcpListener,
};

use libloading::*;

use srvr_sysplugin::Plugin;

use crate::client::Client;

pub mod client;
pub mod task;

fn main() -> Result<(), Box<dyn Error>> {
  println!("Starting Server!");

  //(1) Load plugins
  let lib = unsafe {
  Library::new("target/debug/sample_plugin.dll")
  }.unwrap();

  let linker: Symbol<extern "Rust" fn() -> Box<dyn Plugin>> = unsafe {
  lib.get(b"link")
  }.unwrap();

  let mut plugin = linker();
  plugin.as_mut().start();
  plugin.as_mut().start();

  //(2) Connect to port
  let socket = TcpListener::bind("127.0.0.1:25565")?;
  loop {
    // (1) Open connection
    let (mut stream, addr) = socket.accept()?;
    let client = Client::new(stream, addr);
  }

}