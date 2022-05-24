use std::{
    error::Error,
    net::TcpListener,
};

use libloading::*;
use srvr_sysplugin::Plugin;
use srvr_sysproto::{
  packets::{SBHandshakePacket, Packet},
  raw_packet::RawPacketReader
};

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
    /*
    let mut stream = socket.accept()?.0;
    let mut bytes = vec![0u8;128];

    stream.read(&mut bytes)?;
    let mut reader = RawPacketReader::from_raw(bytes);

    let format = PacketFormat::decode(&mut reader)?;
    println!("Got package in format {:?}", format);
    let handshake = HandshakePacket::decode(&mut reader)?;
    println!("Got handshake: {:?}", handshake);

    */
    let mut stream = socket.accept()?.0;
    let mut reader = RawPacketReader::new(stream)?;
    let handshake = SBHandshakePacket::decode(&mut reader)?;
    println!("Got handshake: {:?}", handshake);
  }
  Ok(())
}