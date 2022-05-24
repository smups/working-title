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

use byteorder::{BigEndian, ByteOrder};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct MCUShort(pub u16);

impl MCDataType for MCUShort {

  fn decode(buf: &mut RawPacketReader) -> Result<MCUShort, Err> {
    Ok(MCUShort(BigEndian::read_u16(&buf.read_bytes(2))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&u16::to_be_bytes((*self).into()))
  }
}

impl From<u16> for MCUShort{
  fn from(val: u16) -> Self {MCUShort(val)}
}

impl From<MCUShort> for u16 {
  fn from(val: MCUShort) -> Self {val.0}
}