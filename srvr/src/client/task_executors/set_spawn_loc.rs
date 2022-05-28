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
  net::TcpStream,
  error::Error
};

use srvr_sysproto::{
  raw_packet::RawPacketWriter,
  packets::{Packet, CB_SpawnPosition}
};

use crate::task::{TaskContext, Task};

pub fn set_spawn_loc(
  ctx: TaskContext,
  stream: &mut TcpStream,
  _: &mut Vec<Task> //No follow-up for this packet
)
  -> Result<(), Box<dyn Error>>
{
  println!("Setting player loc {ctx:?}");
  //(0) Unwrap the context
  let (location, angle) = match ctx {
    TaskContext::SpawnLocCtx{ location, angle } => (location, angle),
    _ => panic!() //invalid context
  };

  //(1) We have to send the spawn location of the player
  let pckt = CB_SpawnPosition{location: location, angle: angle};
  let mut writer = RawPacketWriter::new(pckt.packet_id());
  pckt.encode(&mut writer);
  writer.write(stream)?;

  Ok(())
}