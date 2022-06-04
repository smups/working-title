use srvr_sysworldgen::{
  GenDyLib,
  link_generator,
  chunk::Chunk
};

use log::info;

#[derive(Debug)]
pub struct SuperFlatGenerator {

}

impl GenDyLib for SuperFlatGenerator {
  fn one_time_init(&mut self) {
    todo!()
  }

  fn gen_chunk(&self, pos: (i32, i32, i16)) ->Chunk {
    todo!()
  }
}

impl SuperFlatGenerator {
  pub fn new() -> Self {
    info!("Hello!");
    println!("dis is worldbuld");
    SuperFlatGenerator {}
  }
}

link_generator!(SuperFlatGenerator);