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

use std::{
  error::Error,
  sync::{Mutex, Arc}
};

#[derive(Debug, Clone)]
pub struct Wire<T> where T: Send + Clone {
  queues: Vec<Arc<Mutex<Vec<T>>>>
}

impl<T: 'static> Wire<T> where T: Send + Clone {

  pub fn send(&'static mut self, msg: T) -> Result<(), Box<dyn Error>> {
    for queue in &self.queues {
      match queue.lock() {
        Ok(mut guard) => guard.push(msg.clone()),
        Err(err) => return Err(Box::new(err))
      }
    }
    Ok(())
  }

  pub fn connect(&mut self) -> WireListener<T> {
    let queue = Arc::new(Mutex::new(Vec::new()));
    self.queues.push(queue.clone());
    WireListener{ queue: queue }
  }

  pub fn new() -> Wire<T> {
    Wire { queues: Vec::new() }
  }
}

#[derive(Debug, Clone)]
pub struct WireListener<T> where T: Send {
  queue: Arc<Mutex<Vec<T>>>
}

impl<T> WireListener<T> where T: Send + Clone {
  pub fn poll(&mut self) -> Option<T> {
    self.queue.lock().unwrap().pop()
  }
}