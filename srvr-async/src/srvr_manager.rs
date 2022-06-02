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
  error::Error,
  net::{SocketAddr, Ipv4Addr, IpAddr},
  time::Duration
};

use log::{info, warn};
use tokio::{
  net::TcpListener,
  sync::{broadcast, mpsc}, time::timeout
};

use crate::{
  messages::{
    broadcast::BroadcastMsg,
    client_request::{ClientRequest, CReqMsg, CReqRsp}
  },
  config::Config,
  client::Client,
  console::Console,
};

const MAX_QUEUE_LEN: usize = 100;
const TCP_TIMEOUT: Duration = Duration::from_millis(10);
const TASK_TIMEOUT: Duration = Duration::from_millis(1);

#[derive(Debug)]
pub struct Main {
  config: Config,
  socket: TcpListener,
  broadcast: broadcast::Sender<BroadcastMsg>,
  request_queue: mpsc::Receiver<ClientRequest>,
  request_queue_tx: mpsc::Sender<ClientRequest>,
  clients: Vec<Client>
}

impl Main {

  pub async fn init() -> Result<Self, Box<dyn Error>> {
    //(1) Get global config
    let config = crate::config::copy_config();

    //(2) Try to listen on the port
    let ip: IpAddr = Ipv4Addr::from(config.network_settings.ip).into();
    let socket_addr = SocketAddr::new(ip, config.network_settings.port);
    let socket = TcpListener::bind(socket_addr.clone()).await?;

    //(3) Set up the connections to and from the client
    let (broadcast, _) = broadcast::channel(MAX_QUEUE_LEN);
    let (tx, request_queue) = mpsc::channel(MAX_QUEUE_LEN);
  
    //(R) before we return, say hi to the console
    info!("Server listening @{}", socket_addr);
    Ok(Main {
      config: config,
      socket: socket,
      broadcast: broadcast,
      request_queue: request_queue,
      request_queue_tx: tx,
      clients: Vec::new()
    })
  }

  pub fn connect_console(&mut self) -> Console {
    Console::init(self.request_queue_tx.clone())
  }

  pub async fn run(&mut self) {
    'server_tick: loop {
      /*(1)
        Look for clients. If no client connects within the time-out period,
        we continue to the next step in executing the server-tick.
      */
      if let Ok(fut) = timeout(TCP_TIMEOUT, self.socket.accept()).await {
        //(1a) A client connected! Let's unwrap:
        let (connection, addr) = match fut {
          Ok((connection, addr)) => (connection, addr),
          Err(err) => {
            warn!("could not accept client connection: \"{err}\"");
            return;
          }
        };

        //(1b) Say hi to the console and initialise the client
        if let Some(client) = Client::init(
          connection,
          addr,
          self.broadcast.subscribe(),
          self.request_queue_tx.clone()
        ).await{ self.clients.push(client); }
      }

      /*(2)
        Next we execute the server tick.
      */
      while let Ok(maybe) = timeout(TASK_TIMEOUT, self.request_queue.recv()).await {
        if let Some(request) = maybe {
          use CReqMsg::*;
          //(2a) open the request packet
          let (msg, tx) = request.open();

          //(2b) match the message and answer
          match msg {
            ConsoleKill => {
              //User has killed the server with a console command
              tx.send(Ok(CReqRsp::Done)).unwrap();
              self.shutdown().await;
              break 'server_tick;
            }
          }
        }
      }
    }
  }

  pub async fn shutdown(&mut self) {
    info!("Shutting down...");
  }

}