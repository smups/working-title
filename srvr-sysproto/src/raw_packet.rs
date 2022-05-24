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

#[derive(Debug, Clone, PartialEq, Eq)]
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

  pub fn to_raw(self) -> Vec<u8> {self.bytes}
  pub fn raw_view(&self) -> &Vec<u8> {&self.bytes}

  pub fn read_byte(&mut self) -> u8 {
    let rtrn = self.bytes[self.ptr];
    self.ptr += 1;
    return rtrn;
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

impl From<RawPacketReader> for Vec<u8> {
  fn from(rpw: RawPacketReader) -> Self {rpw.bytes}
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

  pub fn from_raw(raw: Vec<u8>) -> RawPacketWriter {
    let ptr_start = raw.len();
    RawPacketWriter { bytes: raw, ptr: ptr_start}
  }

  pub fn to_raw(self) -> Vec<u8> {self.bytes}
  pub fn raw_view(&self) -> &Vec<u8> {&self.bytes}

  pub fn write_byte(&mut self, byte: u8) {
    self.bytes.push(byte);
    self.ptr += 1;
  }

  pub fn write_bytes(&mut self, bytes: &[u8]) {
    let bytes_written = bytes.len();
    self.bytes.copy_from_slice(bytes);

    //Advance ptr
    self.ptr += bytes_written;
  }

}

impl From<RawPacketWriter> for Vec<u8> {
  fn from(rpw: RawPacketWriter) -> Self {rpw.bytes}
}