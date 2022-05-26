/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.
  
  srvr is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.
  
  You should have received a copy of the GNU General Public License
  along with srvr.  If not, see <http://www.gnu.org/licenses/>.
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