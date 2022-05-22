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

#[derive(Debug, Clone)]
pub struct RawPacketReader {
  /*
    A RawPacket struct contains the raw bytes of a packet. It is later converted
    to a protocol-specific readable format. To aid in this conversion, the RawPacket
    keeps track of how many bytes have been read.
  */
  bytes: Vec<u8>,
  ptr: usize
}

impl RawPacketReader {

  pub fn from_raw(raw: Vec<u8>) -> RawPacketReader {
    RawPacketReader {bytes: raw, ptr: 0}
  }

  pub fn read_bytes(&mut self, num_bytes: usize) -> Vec<u8> {
    let bytes = self.bytes[self.ptr..self.ptr+num_bytes].to_vec();

    //Advance the ptr
    self.ptr += num_bytes;

    return bytes;
  }

  pub fn read_into(&mut self, buf: &mut [u8]) {
    let bytes_to_read = buf.len();
    buf.copy_from_slice(&self.bytes[self.ptr..self.ptr+bytes_to_read]);

    //Advance the ptr
    self.ptr += bytes_to_read;
  }

}

#[derive(Debug, Clone)]
pub struct RawPacketWriter {
  /*
    A RawPacket struct contains the raw bytes of a packet. It is later converted
    to a protocol-specific readable format. To aid in this conversion, the RawPacket
    keeps track of how many bytes have been read.
  */
  bytes: Vec<u8>,
  ptr: usize
}

impl RawPacketWriter {

  pub fn new(size: usize) -> RawPacketWriter {
    RawPacketWriter {bytes: Vec::with_capacity(size), ptr: 0}
  }

  pub fn write_bytes(&mut self, bytes: &[u8]) {
    let bytes_written = bytes.len();
    self.bytes.copy_from_slice(bytes);

    //Advance ptr
    self.ptr += bytes_written;
  }

}