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
pub struct MCFloat(f32);

impl MCDataType for MCFloat {

  fn decode(buf: &mut RawPacketReader) -> Result<MCFloat, Err> {
    Ok(MCFloat(BigEndian::read_f32(&buf.read_bytes(4))))
  }

  fn encode(&self, buf: &mut RawPacketWriter) {
    buf.write_bytes(&f32::to_be_bytes((*self).into()))
  }
}

impl From<f32> for MCFloat{
  fn from(val: f32) -> Self {MCFloat(val)}
}

impl From<MCFloat> for f32 {
  fn from(val: MCFloat) -> Self {val.0}
}