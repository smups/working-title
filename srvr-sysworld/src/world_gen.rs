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

use std::{fmt::Debug, sync::Arc};

use log::info;

use crate::builder_config::BuilderConfig;

#[derive(Debug, Clone)]
pub struct WorldGenerator {
  name: String,
  id: u32,
  config: BuilderConfig,
  generator: Arc<Box<dyn GenDyLib>>
}

impl WorldGenerator {
  pub fn new(config: BuilderConfig, generator: Box<dyn GenDyLib>) -> Self {
    info!("Created new world generator \"{}\"", config.general.name);
    WorldGenerator {
      name: config.general.name.clone(),
      id: config.general.id,
      config: config,
      generator: Arc::new(generator)
    }
  }

  pub fn get_name(&self) -> String {self.name.clone()}
}

pub trait GenDyLib: Debug {

}

pub unsafe trait LinkGenDyLib: GenDyLib {
  unsafe extern "Rust" fn link() -> Box<dyn GenDyLib>;
}

#[macro_export]
macro_rules! link_generator {
  ($generator:ident) => {
    use srvr_sysworldgen::LinkGenDyLib;
    unsafe impl LinkGenDyLib for $generator {
      #[no_mangle]
      unsafe extern "Rust" fn link() -> Box<dyn GenDyLib> {
        Box::new($generator::new())
      }
    }
  };
}