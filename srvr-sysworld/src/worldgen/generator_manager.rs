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
  error::Error, collections::HashMap,
  fmt::{Display, Formatter, self}, fs::File, io::Read, env
};

use libloading::{Library, Symbol};

use super::generator_config::WorldGenConfig;
use super::generator_api::BoxedWorldGenerator;

//File names
const WORLD_GEN_CONFIG: &str = "world.toml";
const WORLD_GEN_BIOME: &str = "biome.toml";

pub struct WorldGeneratorManager {
  /*(Description)
    The worldgenerator manager is an unsafe structure from which world generators
    may be created. It manages the memory required for world generator objects
    to function. 
  */
  libraries: HashMap<&'static str, WorldGeneratorLibrary>
}

impl WorldGeneratorManager {

  pub fn new(generators_folder: &Path)
    -> Result<ManuallyDrop<Self>, Box<dyn Error>>
  {
    /*
      Tries to create worldgenerators for all subfolders of "generators_folder".
    */
    todo!()
  }

  pub fn get_generator(&self, gen_name: &str) -> Option<BoxedWorldGenerator> {
    //(1) First check if we have a generator by the name "gen_name"
    if !self.libraries.contains_key(gen_name) {return None}

    //(2) Get the generator's linker and run it
    let generator_lib = &self.libraries.get(gen_name).unwrap().library;
    let generator_object = unsafe {
      //Get the linker from the library
      let linker: Symbol<unsafe extern "Rust" fn() -> *mut ()> = 
        generator_lib.get(super::generator_api::LINKER_SYMBOL).unwrap();

      //Cast the void pointer provided by the linker to a trait object
      let ptr = linker();
      BoxedWorldGenerator::from_raw(ptr)
    };

    //(R) return the generator trait object
    return Some(generator_object);
  }

  fn parse_generator(folder: &Path) -> Result<WorldGeneratorLibrary, GBErr> {
    /*(1)
      We start with finding and parsing the configuration of the world generator.
      This is contained at the root of the generator's folder, and found in the
      world.toml file.
    */
    let mut config_path = PathBuf::from(folder);
    config_path.push(WORLD_GEN_CONFIG);
    let config = Self::parse_config(&config_path)?;

    /*(2)
      Next we'll load the dynamic library containing the actual world generator
      implementation. The path of the world-gen binary is specified in the config.
    */
    let mut dylib_path = Self::get_lib_path(folder, &config.general.dylib_generator);
    let mut dylib = match unsafe { Library::new(dylib_path) } {
      Ok(dylib) => dylib,
      Err(err) => return Err(format!("could not open world generator library. Error: \"{err}\"").into())
    };

    //(3) Last step is to check if the library actually contains the linker func
    let _: Symbol<unsafe extern "Rust" fn() -> *mut ()> = match unsafe {
      dylib.get(super::generator_api::LINKER_SYMBOL)
    } {
      Ok(symbol) => symbol,
      Err(err) => return Err(format!("could not find linker symbol in generator dylib. Error: \"{err}\"").into())
    };

    //(R) an instance of WorldGeneratorLibrary
    return Ok(WorldGeneratorLibrary {
      name: (&config.general.name).clone(),
      config,
      library: dylib
    })
  }

  fn parse_config(config_path: &Path) -> Result<WorldGenConfig, GBErr> {
    //(1) Open the config file
    let mut config_file = match File::open(&config_path) {
      Ok(f) => f,
      Err(err) => {
        return Err(format!("could not open {} file. Error: \"{err}\"", config_path.display()).into());
      }
    };

    //(2) Try to read the config file
    let mut config_string = String::new();
    if let Err(err) = config_file.read_to_string(&mut config_string) {
      return Err(format!("could not read {} file. Error: \"{err}\"", config_path.display()).into());
    }

    //(3) Try to parse the config file
    match toml::from_str::<WorldGenConfig>(&config_string) {
      Ok(config) => Ok(config),
      Err(err) => Err(format!("could not parse {} file. Error: \"{err}\"", config_path.display()).into())
    }
  }

  fn get_lib_path(folder: &Path, lib_name: &str) -> PathBuf {
    //(1) Copy path root
    let mut root = PathBuf::from(folder);
    
    /*(2)
      Construct the platform-specific name of the executable. This consits of
      an (optional) prefix and an (optional) suffix, with the library name squished
      in between.
    */
    let mut lib_fname = String::from(env::consts::DLL_PREFIX);
    lib_fname.push_str(lib_name);
    lib_fname.push_str(env::consts::DLL_SUFFIX);

    //(R) the folder
    root.push(&lib_fname);
    return root;
  }

}

struct WorldGeneratorLibrary {
  name: String,
  config: WorldGenConfig,
  library: Library
}

#[derive(Debug)]
pub struct GeneratorBuilderError(String);
type GBErr = GeneratorBuilderError;

impl From<String> for GBErr {
  fn from(msg: String) -> Self { Self(msg) }
}
impl From<&str> for GBErr {
  fn from(msg: &str) -> Self { Self(msg.to_string()) }
}

impl Error for GBErr {}
impl Display for GBErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}