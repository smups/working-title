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