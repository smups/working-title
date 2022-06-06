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

/*
  This crate contains auto-generated Rust code for all sorts of MineCraft data.
  Basically, whenever it is extremely awful to implement stuff manually, it
  should be generated (like with block states, the global pallete, biomes
  etc...).

  I decided to define shared 

  Generated code is located in the generated folder. The generators themselves
  are located in the scripts folder.
*/

#![allow(dead_code)]
#![allow(unreachable_patterns)]

//Container for generated code
pub mod generated;

//Traits defining common behaviour of generated code
pub mod block;