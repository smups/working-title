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
  net::{SocketAddr, Ipv4Addr, IpAddr}
};

use log::info;
use tokio::net::TcpListener;

use crate::config::Config;

#[derive(Debug)]
pub struct Main {
  config: Config,
  socket: TcpListener
}

impl Main {

  pub async fn init() -> Result<Self, Box<dyn Error>> {
    //(1) Get global config
    let config = crate::config::copy_config();

    //(2) Try to listen on the port
    let ip: IpAddr = Ipv4Addr::from(config.network_settings.ip).into();
    let socket_addr = SocketAddr::new(ip, config.network_settings.port);
    let socket = TcpListener::bind(socket_addr.clone()).await?;
  
    //(R) before we return, say hi to the console
    info!("Server listening @{}", socket_addr);
    Ok(Main {
      config: config,
      socket: socket
    })
  }

  pub async fn listen(&mut self) {
    loop {
      let (connection, addr) = self.socket.accept().await.unwrap();
      info!("Client connected @{}", addr);
    }
  }

}