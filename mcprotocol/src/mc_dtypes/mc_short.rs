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
pub struct MCShort(i16);

impl MCDataType for MCShort {

  fn decode(buf: &[u8]) -> Result<MCShort, Err> {
    Ok(MCShort(i16::from_be_bytes([buf[0], buf[1]])))
  }

  fn encode(&self, buf: &mut [u8]) {
    i16::to_be_bytes((*self).into())
      .iter()
      .enumerate()
      .for_each(|(index, byte)| buf[index] = *byte);
  }
}

impl From<i16> for MCShort{
  fn from(val: i16) -> Self {MCShort(val)}
}

impl From<MCShort> for i16 {
  fn from(val: MCShort) -> Self {val.0}
}