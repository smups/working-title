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

//! This module contains all the inner workings of the world generation system,
//! but no actual world generators (keep reading to find out why). This is a
//! highly unsafe part of srvr because world generators are provided as dynamic
//! libraries which require a lot of careful manual memory management to prevent
//! segfaults.
//! 
//! # World Generation System
//! As mentioned previously, srvr uses dynamically loaded rust libraries to
//! provide extendable world generation functionality: users can write their own
//! world generators and plug those into the server (similar to how plugins work,
//! just more complicated and unsafe for performance reasons).
//! 
//! The world generation system consists of the following parts:
//! - **World Generation Libraries** worldgen libs are the core of the world
//! generation system in the sense that they actually provide the chunk generation
//! functionality. They are provided as folders the "./world/generators" directory.
//! A valid world generation folder contains the following components:
//!   - a `world.toml` configuration file
//!   - a `biome.toml` file specifying the biomes that the worldgenerator may
//!   create, together with their properties
//!   - and finally a platform-specific rust binary dynamic library file that 
//!   will be loaded by srvr on startup.
//! - **World Generation Library Manager** the worldgen libs are not self-contained
//! after they are loaded. This is because the worldgen libs provide a ffi
//! function to generate an instance of a `BoxedWorldGenerator` thin trait object.
//! The vtables for these thin trait objects must be stored somewhere. Right now,
//! they're stored as static variables in the plugin binaries. Hence the binaries
//! must not be de-allocated before all instances are dropped. This is the job of
//! the world generation library manager.
//! - **World Generators** finally we have the actual world generators themselves.
//! These are just instances of the `BoxedWorldGenerator` trait object. They provide
//! the actual implementation of the `WorldGenerator` trait.


//Modules required to build srvr
#[cfg(feature="worldgen")]
pub mod world_generator;
#[cfg(feature="worldgen")]
pub mod generator_manager;
#[cfg(feature="worldgen")]
pub mod generator_config;

//Modules required to build a world-generator plugin
#[cfg(feature="world_gen_api")]
pub mod generator_api;