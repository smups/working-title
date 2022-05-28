/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 17
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use std::error::Error;

use crate::{
  raw_packet::{RawPacketReader, RawPacketWriter}
};

//Module structure
mod server_bound;
mod client_bound;

pub trait Packet {
  const PACKET_ID: usize;

  fn decode(buf: &mut RawPacketReader) -> Result<Self, Box<dyn Error>> where Self: Sized;
  fn encode(&self, buf: &mut RawPacketWriter);
  fn packet_id(&self) -> usize {Self::PACKET_ID}
}

/*
  Re-export of all Possible server-bound (incoming) packages
*/
//(A) Handshake procedure
pub use server_bound::handshake::HandshakePacket as SB_Handshake;
pub use server_bound::ping::PingPacket as SB_Ping;

//(B) Login procedure
pub use server_bound::login_start::LoginStartPacket as SB_LoginStart;
pub use server_bound::encryption_response::EncryptionResponsePacket as SB_EncryptionResponse;

//(C) Play
pub use server_bound::teleport_confirm::TeleportConfirmPacket as SB_TeleportConfirm;

/*
  Re-export of all Possible client-bound (outgoing) packages
*/
//(A) Handshake procedure
pub use client_bound::status::StatusPacket as CB_Status;
pub use client_bound::pong::PongPacket as CB_Pong;

//(B) Login procedure
pub use client_bound::login_disconnect::LoginDisconnectPacket as CB_LoginDisconnect;
pub use client_bound::encryption_request::EncryptionRequestPacket as CB_EncryptionRequest;
pub use client_bound::login_success::LoginSuccessPacket as CB_LoginSuccess;
pub use client_bound::set_compression::SetCompressionPacket as CB_SetCompression;

//(C) Play
pub use client_bound::join_game::JoinGamePacket as CB_JoinGame;
pub use client_bound::spawn_position::SpawnPositionPacket as CB_SpawnPosition;