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
  path::PathBuf,
  error::Error,
  fmt::{self, Debug, Formatter, Display}, fs::File, io::Read, env
};

use libloading::{Library, Symbol};

use crate::{
  WorldGenerator,
  builder_config::BuilderConfig,
  GenDyLib
};

const WORLD: &'static str = "world.toml";
//const BIOME: &'static str = "biome.toml"; <- don't need this for now

type GBErr = GeneratorBuilderError;

#[derive(Debug)]
pub struct GeneratorBuilder;
impl GeneratorBuilder {

  pub fn build(settings_folder: PathBuf) -> Result<WorldGenerator, GBErr> {
    //(1) Try to parse the configuration file
    let settings = Self::get_settings(settings_folder.clone())?;

    //(2) Try to link to the generator dynamic library
    let dylib_path = Self::get_lib_path(settings_folder, &settings.general.dylib_generator);
    let dylib = Self::link_generator(dylib_path)?;

    Ok(WorldGenerator::new(settings, dylib))
  }

  fn get_settings(settings_folder: PathBuf) -> Result<BuilderConfig, GBErr> {
    //(1) Try to open the config file
    let mut world_settings_path = settings_folder;
    world_settings_path.push(WORLD);

    let mut config_file = match File::open(world_settings_path) {
      Ok(f) => f,
      Err(err) => {
        return Err(format!("could not open {WORLD} file. Error: \"{err}\"").into());
      }
    };

    //(2) Try to read the config file
    let mut config_string = String::new();
    if let Err(err) = config_file.read_to_string(&mut config_string) {
      return Err(format!("could not read {WORLD} file. Error: \"{err}\"").into());
    }

    //(3) Try to parse the config file
    match toml::from_str::<BuilderConfig>(&config_string) {
      Ok(config) => Ok(config),
      Err(err) => Err(format!("could not parse {WORLD} file. Error: \"{err}\"").into())
    }
  }

  fn get_lib_path(mut folder: PathBuf, lib_name: &str) -> PathBuf {
    //(1) Append the platform-specific library prefix
    let mut file_name = String::from(env::consts::DLL_PREFIX);
    //(2) Append the dylib's filename
    file_name.push_str(lib_name);
    //(3) Append the dylib extension
    file_name.push_str(env::consts::DLL_SUFFIX);

    //(R) the folder
    folder.push(&file_name);
    folder
  }

  fn link_generator(dylib_file: PathBuf) -> Result<Box<dyn GenDyLib>, GBErr> {
    //(1) Try to load the dynamic library (eek)
    let lib = match unsafe {Library::new(dylib_file)} {
      Ok(dylib) => dylib,
      Err(err) => return Err(format!("could not load generator dylib. Error\"{err}\"").into())
    };

    //(2) Try to link against the dylib
    let linker: Symbol<extern "Rust" fn() -> Box<dyn GenDyLib>> = match unsafe {
      lib.get(b"link\0")
    } {
      Ok(linker) => linker,
      Err(err) => return Err(format!("could not link to generator dylib. Error\"{err}\"").into())
    };

    //(3) Try to run the linker and return (this sometimes segfaults. Oh well.)
    Ok(linker())
  }

}

#[derive(Debug)]
pub struct GeneratorBuilderError(String);

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