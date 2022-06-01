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

use crate::messages::client_request::{ClientRequest, CReqMsg, CReqRsp, CReqDenied};

const CONSOLE_TICK: Duration = Duration::from_millis(100);

pub struct Console {
  server_handle: mpsc::Sender<ClientRequest>,
  stdin_buf: String,
  buf_parser: fn(&'_ mut String)
}

impl Console {

  pub fn init(server_handle: mpsc::Sender<ClientRequest>) -> Self {
    //(1) Get the OS that we are running on
    let parser = match std::env::consts::OS {
      "linux" => stdin_parse_linux,
      "windows" => stdin_parse_windows,
      _ => stdin_parse_linux
    };

    //(2) Make a console
    Console {
      server_handle: server_handle,
      stdin_buf: String::new(),
      buf_parser: parser
    }
  }

  pub fn run(mut self) {
    /*
      Sadly, stdin is always blocking on most platforms.
      Therefore, we have to run the console on a separate thread from the tokio
      runtime. It is responsible for shutting itself down.
    */
    use CReqMsg::*;

    thread::spawn(move || {
      'console_tick: loop {
        //(1) Read from stdin
        if let Ok(_) = stdin().read_line(&mut self.stdin_buf) {
          //Parse the buffer before matching
          (self.buf_parser)(&mut self.stdin_buf);

          //Try to execute the command
          match self.stdin_buf.as_str() {
            "stop" => {
              //Tell the server to stop
              self.send_msg(ConsoleKill).unwrap();
              //Then stop ourselves
              break 'console_tick;
            },
            other => warn!("Unknown command \"{other}\"")
          }
        };
        //Clean the buffer lol
        self.stdin_buf = String::new();
        thread::sleep(CONSOLE_TICK);
      }
    });
  }

  fn send_msg(&mut self, msg: CReqMsg) -> Result<CReqRsp, CReqDenied>{
    tokio::runtime::Builder::new_current_thread()
      .build()
      .unwrap()
      .block_on(
        ClientRequest::send(msg, self.server_handle.clone())
      )
  }

}

fn stdin_parse_linux(stdin_buf: &mut String) {
  //Linux stdin includes newline which we should pop
  stdin_buf.pop();
}

fn stdin_parse_windows(stdin_buf: &mut String) {
  //Windows stdin includes newline AND carriage return
  stdin_buf.pop();
  stdin_buf.pop();
}