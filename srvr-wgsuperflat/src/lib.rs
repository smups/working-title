use srvr_sysworld::{
  link_generator,
  worldgen::generator_api::*,
  chunk::Chunk
};

const GENERATOR_ID: u8 = 0x00;

#[derive(Debug, Clone)]
pub struct SuperFlatGenerator {
  id: u8
}

impl WorldGenerator for SuperFlatGenerator {
  
  fn one_time_init(&mut self) {
    println!("Hello World!");
  }

  fn gen_chunk(&self, pos: (i32, i32, i16)) -> Chunk {
    Chunk {  }
  }
}

impl SuperFlatGenerator {
  pub fn new() -> Self {
    println!("linklink");
    SuperFlatGenerator { id: GENERATOR_ID }
  }
}

link_generator!(SuperFlatGenerator);