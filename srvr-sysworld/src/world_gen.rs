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

use std::mem::ManuallyDrop;

use thin_trait_object::*;
use log::info;

use crate::{builder_config::WorldGenConfig, chunk::Chunk};

#[derive(Clone)]
pub struct WorldGenerator {
  name: String,
  config: WorldGenConfig,
  generator: BoxedGenDyLib<'static>
}

impl WorldGenerator {

  pub fn new(config: WorldGenConfig, generator: BoxedGenDyLib<'static>) -> Self {
    info!("Linked to world generator \"{}\"", config.general.name);
    WorldGenerator {
      name: config.general.name.clone(),
      config: config,
      generator: generator
    }

  }

  pub fn get_name(&self) -> String {self.name.clone()}

}

#[thin_trait_object]
pub trait GenDyLib {
  unsafe fn one_time_init(&mut self);
  fn gen_chunk(&self, pos: (i32, i32, i16)) -> Chunk;
}

impl Clone for BoxedGenDyLib<'static> {
  fn clone(&self) -> Self {
    /*
      So, this code is super cursed and wrong and leaks memory. Then again, that
      is probably ok in this case.
      
      We're basically cloning the raw pointer to the vtable of this particular
      GenDyLib trait object. These vtables are stored in a static global variable
      (hey, that's cursed too!) and are never dropped (well, not before the
      programme ends). Therefore, it's essentially always ok to create new pointers
      to these vtables, as long as none of these pointers deallocate the vtable
      (that would break everything).
    */
    unsafe {
      let ptr: *mut () = self.as_raw(); // <- WARNING VOID PTR
      let mut do_not_drop = ManuallyDrop::new(BoxedGenDyLib::from_raw(ptr));
      ManuallyDrop::take(&mut do_not_drop) // <- WARNING MEMORY LEAK
    }
  }
}