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

use std::fs::File;

use log::info;

use crate::{
  world_builder::WorldBuilderError,
  worldgen::generator_api::BoxedWorldGenerator
};

#[derive(Debug)]
pub struct World {
  
}

impl World {

  pub fn load(gen: BoxedWorldGenerator, file_handle: File, name: String)
    -> Result<Self, WorldBuilderError>
  {
    info!("Loading world \"{name}\"...");
    todo!()
  }

  pub fn new(gen: BoxedWorldGenerator, file_handle: File, name: String)
  -> Result<Self, WorldBuilderError>
  {
    info!("Creating new world \"{name}\"...");
    todo!();
  }

}