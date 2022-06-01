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

use srvr_sysproto::{raw_packet::RawPacketReader, packets::SB_Handshake};
use tokio::{
  sync::{broadcast, mpsc},
  net::TcpStream
};

use crate::messages::{
  broadcast::BroadcastMsg,
  client_request::ClientRequest
};

mod net;
use net::*;

#[derive(Debug)]
pub struct Client {
  connection: TcpStream,
  broadcast_listener: broadcast::Receiver<BroadcastMsg>,
  superior: mpsc::Sender<ClientRequest>
}

impl Client {

  pub async fn init(
    mut conn: TcpStream,
    broadcast: broadcast::Receiver<BroadcastMsg>,
    server_handle: mpsc::Sender<ClientRequest>
  )
    -> Option<Self>
  {
    //(1) We always start the connection with a handshake packet
    let mut packet = RawPacketReader::read(&mut conn).await.unwrap();
    let next = match packet.get_package_id() {
      0x00 => x00_handshake::handle_package(packet, &mut conn).await,
      _ => {return None;} //do nothing
    };

    Some(Client {
      connection: conn,
      broadcast_listener: broadcast,
      superior: server_handle
    })
  }

}