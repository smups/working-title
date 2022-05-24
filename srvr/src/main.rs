use std::{error::Error, net::{TcpStream, TcpListener}, io::Read};

use libloading::*;
use srvr_sysplugin::Plugin;
use srvr_sysproto::{
    packets::{HandshakePacket, Packet, PacketFormat},
    raw_packet::RawPacketReader, mc_dtypes::MCDataType
};

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
    plugin.as_mut().start();

    //(2) Connect to port
    let socket = TcpListener::bind("127.0.0.1:25565")?;
    let mut stream = socket.accept()?.0;
    let mut bytes = vec![0u8;128];

    stream.read(&mut bytes)?;
    let mut reader = RawPacketReader::from_raw(bytes);

    let format = PacketFormat::decode(&mut reader)?;
    println!("Got package in format {:?}", format);
    let handshake = HandshakePacket::decode(&mut reader)?;
    println!("Got handshake: {:?}", handshake);
    
    Ok(())
}