use std::error::Error;

use libloading::*;
use srvr_sysplugin::Plugin;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Server!");

    let lib = unsafe {
        Library::new("target/debug/libsample_plugin.so")
    }.unwrap();

    let linker: Symbol<extern "Rust" fn() -> Box<dyn Plugin>> = unsafe {
        lib.get(b"link")
    }.unwrap();

    let mut plugin = linker();
    plugin.as_mut().start();
    plugin.as_mut().start();

    Ok(())
}