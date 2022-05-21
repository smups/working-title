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

#[derive(Debug, Clone, Copy)]
pub struct MCInt(i32);

impl MCDataType for MCInt {

  fn decode(buf: &[u8]) -> Result<MCInt, Err> {
    Ok(MCInt(i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]])))
  }

  fn encode(&self, buf: &mut [u8]) {
    i32::to_be_bytes((*self).into())
      .iter()
      .enumerate()
      .for_each(|(index, byte)| buf[index] = *byte);
  }
}

impl From<i32> for MCInt{
  fn from(val: i32) -> Self {MCInt(val)}
}

impl From<MCInt> for i32 {
  fn from(val: MCInt) -> Self {val.0}
}