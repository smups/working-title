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

use super::*;

#[derive(Debug, Clone)]
pub struct MCArray<T: MCDataType>(Vec<T>);

impl<T> MCArray<T> where T: MCDataType {

  pub fn decode(len: usize, reader: &mut RawPacketReader)
    -> Result<MCArray<T>, Err>
  {
    let mut array = Vec::new();
    for _ in 0..len {
      array.push(T::decode(reader)?)
    }
    Ok(MCArray(array))
  }

  pub fn encode(&self, writer: &mut RawPacketWriter) {
    for item in &self.0 {
      item.encode(writer);
    }
  }

}

impl<T> From<Vec<T>> for MCArray<T> where T: MCDataType {
  fn from(val: Vec<T>) -> Self {MCArray(val)}
}

impl<T> From<MCArray<T>> for Vec<T> where T: MCDataType {
  fn from(arr: MCArray<T>) -> Self {arr.0}
}