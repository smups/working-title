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

use std::net::SocketAddr;

use log::{warn, info};
use rand::Rng;
use srvr_sysproto::{raw_packet::RawPacketReader};
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
  client_id: u128,
  connection: TcpStream,
  addr: SocketAddr,
  broadcast_listener: broadcast::Receiver<BroadcastMsg>,
  superior: mpsc::Sender<ClientRequest>
}

impl Client {

  pub fn get_id(&self) -> u128 {self.client_id}

  pub async fn init(
    mut conn: TcpStream,
    addr: SocketAddr,
    broadcast: broadcast::Receiver<BroadcastMsg>,
    server_handle: mpsc::Sender<ClientRequest>
  )
    -> Option<Self>
  {
    info!("Client connected @{}", addr);
    /*(Note to future self)
      A client that just connected can want one of three things:
        (1) Do a Handshake, possibly followed by a ping
        (2) Do a ping or a server-list ping (without a handshake)
        (3) Join the game
      Modern minecraft clients don't ever do (2) and place pretty strict bounds
      on how quickly the server must respond with a pong packet in the case they
      send a ping packet, since it is meant to measure the connection speed.

      In fact, we can't close the connection after the handshake packet in case
      the client wants to send a ping packet, even if the client drops the
      connection (which vanilla clients do) because re-opening the connection to
      receive the ping packet takes too long.

      Therefore, we only close the connection if the reader has confirmed 5 times
      that the client has indeed disconnected.
    */
    let mut disconnect_counter = 0usize;

    'handshake: loop {
      let packet = RawPacketReader::read(&mut conn).await.unwrap();
      match packet.get_package_id() {
        0x00 => {
          //If the client indicates he wants to login, we break the loop
          if let 0x02 = x00_handshake::handle_package(packet, &mut conn).await {
            break 'handshake;
          }
        },
        0x01 => {
          //We answer the ping with a pong, then drop the connection
          x01_pingpong::handle_package(packet, &mut conn).await;
          info!("Ping-Pong! Client disconnected @{}", &addr);
          return None;
        },
        0xfe => {}, //legacy ping, not implemented for now
        usize::MAX if disconnect_counter >= 5 => {
          //Connection was dropped 5 times, give up on this client
          info!("Client disconnected @{}", &addr);
          return None;
        },
        usize::MAX => disconnect_counter += 1,
        invalid_opcode => {
          //Invalid Opcode
          warn!("Client @{} sent invalid opcode {invalid_opcode:#04x}", &addr);
          return None;
        }
      }
    }

    //We broke out of the loop because the client wants to login and then play
    //We hand control of the client over to the server manager
    Some(Client {
      client_id: rand::thread_rng().gen(),
      connection: conn,
      addr: addr,
      broadcast_listener: broadcast,
      superior: server_handle
    })
  }

  pub fn login(mut self) {
    /*(Note to future self)
      We start the login process right here
    */
    tokio::spawn(async move {
      //(1) We start the login loop
      info!("Login Request from Client @{}", &self.addr);
      let username;

      'login: loop {
        let packet = RawPacketReader::read(&mut self.connection).await.unwrap();
        match packet.get_package_id() {
          0x00 => {
            //We handle the login request and break the loop to continue to the
            //play phase
            username = x00_login::handle_package(packet, &mut self.connection).await;
            break 'login;
          }
          usize::MAX => {
            //Client has disconnected, we should NOT try to proceed to play phase
            info!("Client disconnected @{}", &self.addr);
            return;
          },
          invalid_opcode => {
            //Invalid Opcode
            warn!("Client @{} sent invalid opcode {invalid_opcode:#04x}", &self.addr);
          }
        }
      }

      //(2) Loop was broken so login was successful! We may continue to the play
      //phase
      self.play(username).await;
    });
  }

  async fn play(mut self, username: String) {
    info!("Player \"{username}\" joined the game!");
    info!("Client disconnected @{}", &self.addr);
  }

}