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
pub struct MCShort(i16);

impl MCDataType for MCShort {

  fn decode(buf: &mut RawPacketReader) -> Result<MCShort, Err> {
    Ok(MCShort(BigEndian::read_i16(&buf.read_bytes(2))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&i16::to_be_bytes((*self).into()))
  }
}

impl From<i16> for MCShort{
  fn from(val: i16) -> Self {MCShort(val)}
}

impl From<MCShort> for i16 {
  fn from(val: MCShort) -> Self {val.0}
}