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

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderConfig {
  pub general: GeneralSettings,
  pub world_gen: WorldGenSettings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
  pub id: u32,
  pub name: String,
  pub dylib_generator: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenSettings {
  pub ambient_light: f32,
  pub bed_works: bool,
  pub coordinate_scale: f64,
  pub effects: String,
  pub has_ceiling: bool,
  pub has_raids: bool,
  pub has_skylight: bool,
  pub height: i32,
  pub min_y: i32,
  pub infiniburn: String,
  pub local_height: i32,
  pub natural: bool,
  pub piglin_safe: bool,
  pub respawn_anchor_works: bool,
  pub ultrawarm: bool
}