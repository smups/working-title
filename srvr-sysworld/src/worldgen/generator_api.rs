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

//! Unsafe FFI API used to pass world generators between worldgen plugins and
//! the main server instance. Should not be touched too often if possible, since
//! any change in the API *will* break all plugins.
//! 
//! More details are provided in module-level documentation.

use std::fmt::{self, Debug, Formatter};
use thin_trait_object::thin_trait_object;
use crate::chunk::Chunk;

pub const LINKER_SYMBOL: &[u8; 5] = b"link\0";

/// Trait for trait objects to be passed over the srvr-worldgenerator plugin ffi
/// boundary. Normal trait objects are not memsafe, so we use a thin trait object
/// (essentially a manually implemented trait object) to pass implementations.
/// 
/// This trait is technically not suitable for ffi since the vtable it generates
/// is not repr(C). Right now this works though, so I probably will not change
/// it unless I absolutely have to.
#[thin_trait_object]
pub trait WorldGenerator: Debug {
  fn one_time_init(&mut self);
  fn gen_chunk(&self, pos: (i32, i32, i16)) -> Chunk;
}

//default impl of debug
impl Debug for BoxedWorldGenerator<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("BoxedWorldGenerator").field(&self.0).finish()
  }
}

/// Trait implemented by world generators to link the generator's dylib. The name
/// of this function should match that of the linker symbol, as defined in the
/// constant above.
/// Safety: *mut () must point to a heap-allocated `BoxedWorldGenerator` instance
pub unsafe trait LinkGeneratorDyLib: WorldGenerator + Clone {
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