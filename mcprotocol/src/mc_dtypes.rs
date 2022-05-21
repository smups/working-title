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
use std::{
  error::Error,
  fmt::{Display, Formatter}
};

//List of all the mc datatypes
mod mc_bool;
mod mc_byte;
mod mc_ubyte;
mod mc_short;
mod mc_ushort;
mod mc_int;
mod mc_long;

pub trait MCDataType {
  fn decode(buf: &[u8]) -> Result<Self, Err> where Self: Sized;
  fn encode(&self, buf: &mut [u8]);
}

#[derive(Debug)]
pub struct MCDataTypeDecodeError(String);
type Err = MCDataTypeDecodeError;

impl Display for MCDataTypeDecodeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while decoding MCDataType: '{}'", self.0)
  }
}

impl Error for MCDataTypeDecodeError{}

impl From<Box<dyn Error>> for MCDataTypeDecodeError {
    fn from(err: Box<dyn Error>) -> Self {
        MCDataTypeDecodeError(format!("{err}"))
    }
}