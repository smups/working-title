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

use std::{fs, error::Error};

//Internal deps
use config::Config;
use srvr_sysworldgen::{
  self,
  WorldGenerator,
  world_builder::WorldBuilder
};

//External deps
use tokio::runtime::Builder;
pub use log::*;
pub use semver::Version;

//Private modules
mod logger;
mod config;
mod client;
mod console;
mod srvr_manager;

//Public modules
pub mod messages;

//Version
pub const VERSION: Version = Version::new(0,0,1);

//Folders and files
pub const LOG_FOLDER: &'static str = "./logs";
pub const PLUGIN_FOLDER: &'static str = "./plugins";
pub const CONFIG_FILE: &'static str = "./config.toml";
pub const WORLD_BUILDER_FOLDER: &'static str = "./world/generators";

//Public configuration file
pub static mut CONFIG: Option<Config> = None;

fn main() {
  //(1) Very first task is to set-up the logging system
  logger::start_logger();

  //(2) Load the configuration and store it into the global static var
  match config::load_config() {
    Ok(config) => {
      info!("Finished loading config file");
      unsafe { CONFIG = Some(config) }
    } Err(err) => {
      error!("Could not parse config file (reason: \"{}\"). Shutting down...", err);
      return;
    }
  };
  let config = config::copy_config();

  //(3) Look in the world builder folder and try to turn each entry into a world
  //builder
  info!("Loading worldbuilders...");
  let builder_folder = match fs::read_dir(WORLD_BUILDER_FOLDER) {
    Ok(folder) => folder,
    Err(err) => {
      error!("Could not open worldbuilder folder (reason: \"{err}\"");
      return;
    }
  };
  let builders: Vec<WorldGenerator> = builder_folder.into_iter()
    .filter(|entry| entry.is_ok())
    .filter(|entry| entry.as_ref().unwrap().path().is_dir())
    .map(|entry| entry.unwrap().path())
    .map(|path| -> Option<WorldGenerator> {
      let generator = WorldBuilder::build(path.clone());
      match generator {
        Ok(generator) => Some(generator),
        Err(err) => {
          warn!("Could not create generator from folder {path:#?}. Reason: \"{err}\"");
          None
        }
      }
    })
    .filter(|builder| builder.is_some())
    .map(|builder| builder.unwrap())
    .collect();

  //(3) Set-up the async threadpool
  let runtime = match Builder::new_multi_thread()
    .worker_threads(config.general_settings.async_workers)
    .max_blocking_threads(config.general_settings.blocking_workers)
    .thread_stack_size(config.general_settings.stack_size)
    .enable_all()
    .thread_name("srvr-worker")
    .build()
  {
    Ok(runtime) => {
      info!("Finished setting up runtime");
      runtime
    } Err(err) => {
      error!("Could not set-up runtime (reason: \"{}\"). Shutting down...", err);
      return;
    }
  };

  //(4) Load plugins (oof!)

  //(5) Start Runtime
  runtime.block_on(async {
    match srvr_manager::Main::init().await {
      Ok(mut srvr) => {
        //(6) Initialise the Console
        srvr.connect_console().run();

        //(7) Start the server
        info!("Startup complete!");
        srvr.run().await;
      } Err(err) => {
        error!("Could not bind to server socket (reason: \"{}\"). Shutting down...", err);
      }
    }
  });
}