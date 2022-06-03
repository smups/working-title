use srvr_sysworldgen::{GenDyLib, link_generator};

use log::info;

#[derive(Debug)]
pub struct SuperFlatGenerator {

}

impl GenDyLib for SuperFlatGenerator {

}

impl SuperFlatGenerator {
  pub fn new() -> Self {
    info!("Hello!");
    println!("dis is worldbuld");
    SuperFlatGenerator {}
  }
}

link_generator!(SuperFlatGenerator);