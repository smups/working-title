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

use std::{fs::File, io::Read, error::Error};

use serde::{Serialize, Deserialize};

pub fn load_config() -> Result<crate::config::Config, Box<dyn Error>> {
  //(1) Try to open the config file
  let mut config_file = File::open(super::CONFIG_FILE)?;

  //(2) Try to read the config file
  let mut config_string = String::new();
  config_file.read_to_string(&mut config_string)?;
  let config: Config = toml::from_str(&config_string)?;

  //(R) Return the config
  Ok(config)
}

pub fn copy_config() -> Config {
  unsafe { crate::CONFIG.clone().unwrap() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub general_settings: GeneralSettings,
  pub network_settings: NetworkSettings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
  pub async_workers: usize,
  pub blocking_workers: usize,
  pub stack_size: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
  pub ip: [u8; 4],
  pub port: u16
}