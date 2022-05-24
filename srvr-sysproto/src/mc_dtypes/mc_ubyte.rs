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
pub struct MCUByte(u8);

impl MCDataType for MCUByte {

  fn decode(buf: &mut RawPacketReader) -> Result<MCUByte, Err> {
    Ok(MCUByte(u8::from_be_bytes([buf.read_bytes(1)[0]])))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&u8::to_be_bytes((*self).into()))
  }
}

impl From<u8> for MCUByte{
  fn from(val: u8) -> Self {MCUByte(val)}
}

impl From<MCUByte> for u8 {
  fn from(val: MCUByte) -> Self {val.0}
}