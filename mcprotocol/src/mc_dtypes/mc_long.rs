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
pub struct MCLong(i64);

impl MCDataType for MCLong {

  fn decode(buf: &[u8]) -> Result<MCLong, Err> {
    Ok(MCLong(i64::from_be_bytes([
      buf[0], buf[1], buf[2], buf[3],
      buf[4], buf[5], buf[6], buf[7]
      ])))
  }

  fn encode(&self, buf: &mut [u8]) {
    i64::to_be_bytes((*self).into())
      .iter()
      .enumerate()
      .for_each(|(index, byte)| buf[index] = *byte);
  }
}

impl From<i64> for MCLong{
  fn from(val: i64) -> Self {MCLong(val)}
}

impl From<MCLong> for i64 {
  fn from(val: MCLong) -> Self {val.0}
}