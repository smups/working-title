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

use std::fmt::Debug;

use thin_trait_object::thin_trait_object;

use crate::chunk::Chunk;

#[thin_trait_object]
pub trait WorldGenerator: Debug {
  fn one_time_init(&mut self);
  fn gen_chunk(&self, pos: (i32, i32, i16)) -> Chunk;
}

/*
  Trait implemented by generators
*/
pub unsafe trait LinkGeneratorDyLib: GenDyLib + Clone {
  unsafe extern "Rust" fn link() -> *mut ();
}

#[macro_export]
macro_rules! link_generator {
  ($generator:ident) => {
    use srvr_sysworld::worldgen::generator_api::{
      LinkGeneratorDyLib, BoxedWorldGenerator
    };
    unsafe impl LinkGeneratorDyLib for $generator {
      #[no_mangle]
      unsafe extern "Rust" fn link() -> *mut () {
        BoxedWorldGenerator::new($generator::new()).into_raw()
      }
    }
  };
}