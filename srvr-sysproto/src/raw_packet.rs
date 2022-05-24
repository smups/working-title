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

use std::{net::TcpStream, io::Read, error::Error};

use crate::mc_dtypes::{MCVarInt, MCDataType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawPacketReader {
  /*
    A RawPacket struct contains the raw bytes of a packet. It is later converted
    to a protocol-specific readable format. To aid in this conversion, the RawPacket
    keeps track of how many bytes have been read.
  */
  data: Vec<u8>,
  ptr: usize,
  id: usize
}

impl RawPacketReader {

  pub fn new(mut stream: TcpStream) -> Result<Self, Box<dyn Error>> {
    /*(1)
      We first have to find the length of the package. This is encoded in an
      up-to 3-byte varint. If we read too many bytes, we'll have to account for
      that.
    */
    let mut tmp_buf = vec![0u8;3];
    stream.read(&mut tmp_buf)?;
    let mut tmp_reader= RawPacketReader::from_raw(tmp_buf);

    let package_len: i32 = MCVarInt::decode(&mut tmp_reader)?.into();
    let excess = 3 - tmp_reader.ptr; //bytes that are not part of package_len
    let bytes_to_read = (package_len as usize) - excess;

    //(2) Read the remaining contents
    let mut big_buf = vec![0u8;bytes_to_read];
    stream.read(&mut big_buf)?;
    let mut reader = match excess {
      0 => RawPacketReader::from_raw(big_buf),
      1 | 2 => {
        let mut complete_buf = tmp_reader.to_raw()[3-excess..3].to_vec();
        complete_buf.append(&mut big_buf);
        RawPacketReader::from_raw(complete_buf)
      } other => {
        todo!();
      }
    };

    //(3) Set the package ID
    let package_id: i32 = MCVarInt::decode(&mut reader)?.into();
    reader.id = package_id as usize;

    Ok(reader)
  }

  pub fn from_raw(raw: Vec<u8>) -> RawPacketReader {
    RawPacketReader {data: raw, ptr: 0, id: 0}
  }

  pub fn to_raw(self) -> Vec<u8> {self.data}
  pub fn raw_view(&self) -> &Vec<u8> {&self.data}

  pub fn read_byte(&mut self) -> u8 {
    let rtrn = self.data[self.ptr];
    self.ptr += 1;
    return rtrn;
  }

  pub fn read_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
    let data = self.data[self.ptr..self.ptr+num_bytes].to_vec();

    //Advance the ptr
    self.ptr += num_bytes;

    return data;
  }

  pub fn read_into(&mut self, buf: &mut [u8]) {
    let bytes_to_read = buf.len();
    buf.copy_from_slice(&self.data[self.ptr..self.ptr+bytes_to_read]);

    //Advance the ptr
    self.ptr += bytes_to_read;
  }

}

impl From<RawPacketReader> for Vec<u8> {
  fn from(rpw: RawPacketReader) -> Self {rpw.data}
}

#[derive(Debug, Clone)]
pub struct RawPacketWriter {
  /*
    A RawPacket struct contains the raw bytes of a packet. It is later converted
    to a protocol-specific readable format. To aid in this conversion, the RawPacket
    keeps track of how many bytes have been read.
  */
  bytes: Vec<u8>
}

impl RawPacketWriter {

  pub fn new(size: usize) -> RawPacketWriter {
    RawPacketWriter {bytes: Vec::with_capacity(size)}
  }

  pub fn from_raw(raw: Vec<u8>) -> RawPacketWriter {
    let ptr_start = raw.len();
    RawPacketWriter { bytes: raw }
  }

  pub fn to_raw(self) -> Vec<u8> {self.bytes}
  pub fn raw_view(&self) -> &Vec<u8> {&self.bytes}

  pub fn write_byte(&mut self, byte: u8) {self.bytes.push(byte);}

  pub fn write_bytes(&mut self, bytes: &[u8]) {
    bytes.iter().for_each(|byte| self.write_byte(*byte));
  }

}

impl From<RawPacketWriter> for Vec<u8> {
  fn from(rpw: RawPacketWriter) -> Self {rpw.bytes}
}