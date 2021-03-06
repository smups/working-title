/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 15
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use std::{
  error::Error}
;

use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream
};

use crate::mc_dtypes::{MCVarInt, MCDataType};

const MAX_PACKAGE_LEN: usize = 2097151;

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

  pub async fn read(stream: &mut TcpStream) -> Result<Self, Box<dyn Error>> {
    /*(1)
      We first have to find the length of the package. This is encoded in an
      up-to 3-byte varint. If we read too many bytes, we'll have to account for
      that.
    */
    let mut tmp_buf = vec![0u8;3];
    stream.read(&mut tmp_buf).await?;
    let mut tmp_reader= RawPacketReader::from_raw(tmp_buf);

    let package_len: i32 = MCVarInt::decode(&mut tmp_reader)?.into();
  
    //Ignore legacy ping packets
    if package_len as usize == tmp_reader.ptr || package_len == 0 {
      return Ok(RawPacketReader{data: Vec::new(), ptr: 0usize, id: usize::MAX})
    }

    let excess = 3 - tmp_reader.ptr; //bytes that are not part of package_len
    let bytes_to_read = (package_len as usize) - excess;

    //(2) Read the remaining contents
    let mut big_buf = vec![0u8;bytes_to_read];
    stream.readable().await?;
    stream.try_read(&mut big_buf)?;
    let mut reader = match excess {
      0 => RawPacketReader::from_raw(big_buf),
      1 | 2 => {
        let mut complete_buf = tmp_reader.to_raw()[3-excess..3].to_vec();
        complete_buf.append(&mut big_buf);
        RawPacketReader::from_raw(complete_buf)
      } other => {
        todo!(); //should error the fuck out
      }
    };

    //(3) Set the package ID
    let package_id: i32 = MCVarInt::decode(&mut reader)?.into();
    reader.id = package_id as usize;

    Ok(reader)
  }

  pub fn get_package_id(&self) -> usize {self.id}

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
  bytes: Vec<u8>,
  id: usize
}

impl RawPacketWriter {

  pub async fn write(mut self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    //(1) First we should encode the package ID, since its length is included
    //in the package length
    let mut tmp_writer = RawPacketWriter::empty(0);
    MCVarInt::from(self.id as i32).encode(&mut tmp_writer);
    let mut id_varint_buf = tmp_writer.to_raw();
    let package_len = self.bytes.len() + id_varint_buf.len();

    //(2) Next, we'll encode the package length
    tmp_writer = RawPacketWriter::empty(0);
    MCVarInt::from(package_len as i32).encode(&mut tmp_writer);
    let mut full_buf = tmp_writer.to_raw();

    //(3) We still have to append the ID and the package data
    full_buf.append(&mut id_varint_buf);
    full_buf.append(&mut self.bytes);

    //(4) Now we write the bytes to the stream
    stream.write(&full_buf).await?;
    Ok(())
  }

  pub fn new(packet_id: usize) -> RawPacketWriter {
    RawPacketWriter { bytes: Vec::new(), id: packet_id }
  }

  fn empty(size: usize) -> RawPacketWriter {
    RawPacketWriter {bytes: Vec::with_capacity(size), id: 0}
  }

  pub fn from_raw(raw: Vec<u8>) -> RawPacketWriter {
    RawPacketWriter { bytes: raw, id: 0 }
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