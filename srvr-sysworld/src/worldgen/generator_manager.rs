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

use std::{
  mem::ManuallyDrop,
  path::{PathBuf, Path},
  error::Error, collections::HashMap
};

use libloading::Library;

use crate::builder_config::WorldGenConfig;

use super::generator_api::BoxedWorldGenerator;

struct WorldGeneratorManager {
  /*(Description)
    The worldgenerator manager is an unsafe structure from which world generators
    may be created. It manages the memory required for world generator objects
    to function. 
  */
  libraries: HashMap<&'static str, WorldGeneratorLibrary>
}

impl WorldGeneratorManager {

  pub fn new(generators_folder: &Path) -> Result<ManuallyDrop<Self>> {
    /*
      Tries to create worldgenerators for all subfolders of "generators_folder".
    */
    todo!()
  }

  pub fn get_generator(&self, gen_name: &str) -> Option<BoxedWorldGenerator> {
    //(1) First check if we have a generator by the name "gen_name"
    if !self.libraries.contains_key(gen_name) {return None}

    //(2) Get the generator's linker and run it
    let generator_linker = self.libraries.get(gen_name).unwrap().linker;
    let generator_object = unsafe {
      //Cast the void pointer provided by the linker to a trait object
      let ptr = generator_linker();
      BoxedWorldGenerator::from_raw(ptr)
    };

    //(R) return the generator trait object
    return Some(generator_object);
  }

  fn parse_generator(folder: &Path)
    -> Result<WorldGeneratorLibrary, Box<dyn Error>>
  {
    /*(1)
      We start with finding and parsing the configuration of the world generator.
      This is contained at the root of the generator's folder
    */
  }
}

struct WorldGeneratorLibrary {
  name: String,
  config: WorldGenConfig,
  library: Library,
  linker: unsafe fn () -> *mut ()
}