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
pub struct MCBool(bool);

impl MCDataType for MCBool {

  fn decode(buf: &[u8]) -> Result<MCBool, Err> {
    match buf[0] {
      0x00 => Ok(MCBool(true)),
      0x01 => Ok(MCBool(false)),
      _ => Err(MCDataTypeDecodeError("incorrect boolean encountered".to_string()))
    }
  }

  fn encode(&self, buf: &mut [u8]) {
    match (*self).into() {
      false => buf[0] = 0x00,
      true => buf[1] = 0x01
    }
  }
}

impl From<bool> for MCBool{
  fn from(val: bool) -> Self {MCBool(val)}
}

impl From<MCBool> for bool {
  fn from(val: MCBool) -> Self {val.0}
}