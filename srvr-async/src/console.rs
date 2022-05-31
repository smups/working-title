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
  thread,
  time::Duration, io::stdin
};

use log::warn;
use tokio::sync::mpsc;

use crate::{
  instructions::ServerInstruction,
};

const CONSOLE_TICK: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct Console {
  server_handle: mpsc::Sender<ServerInstruction>,
  stdin_buf: String
}

impl Console {

  pub fn init(server_handle: mpsc::Sender<ServerInstruction>) -> Self {
    //Connect to the main server, that's all
    Console { server_handle: server_handle, stdin_buf: String::new() }
  }

  pub fn run(mut self) {
    /*
    
    */
    use ServerInstruction::*;

    thread::spawn(move || {
      'console_tick: loop {
        //(1) Read from stdin
        if let Ok(_) = stdin().read_line(&mut self.stdin_buf) {
          //Remove the newline char before continuing!
          self.stdin_buf.pop();

          //Try to execute the command
          match self.stdin_buf.as_str() {
            "stop" => {
              //Tell the server to stop
              self.server_handle.blocking_send(Die).unwrap();
              //Then stop ourselves
              break 'console_tick;
            },
            other => warn!("Unknown command \"{other}\"")
          }
        };
        thread::sleep(CONSOLE_TICK);
      }
    });
  }

}