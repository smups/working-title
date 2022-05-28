/*
  Copyright (C) 2022 Raúl Wolters
  
  This file is part of srvr.
  
  srvr is free software: you can redistribute it and/or modify it under the
  terms of the European Union Public License (EUPL), provided that you publish
  your modifications under the terms of the EUPL or another compatible license
  as specified by the EUPL v1.2 or higher.

  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
  license agreement shall be governed by dutch law, as specified in clause 17
  of the EUPL v1.2.

  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
  
  You should have received a copy of the European Union Public License in a
  official language of the European Union along with srvr. If not, see
  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
  text of the license in any official language of the European Union.
*/

use super::*;

#[derive(Debug, Clone)]
pub enum NbtTag {
  End,
  Byte(Option<String>, i8),
  Short(Option<String>, i16),
  Int(Option<String>, i32),
  Long(Option<String>, i64),
  Float(Option<String>, f32),
  Double(Option<String>, f64),
  ByteArray(Option<String>, Vec<u8>),
  String(Option<String>, String),
  List(Option<String>, Vec<NbtTag>), //UNNAMED!
  Compound(Option<String>, Vec<NbtTag>), //NAMED!
  IntArray(Option<String>, Vec<i32>),
  LongArray(Option<String>, Vec<i64>)
}

/*
  Now some helper functions for encoding/decoding
*/
#[inline]
fn encode_nbt_tag_name(tag_name: Option<String>, writer: &mut RawPacketWriter) {
  //(1) Get length of tag name
  let name_len = match &tag_name {
    None => 0,
    Some(name) => name.len()
  };

  //(2) Write name length prefix as unsigned 16-bit big endian int
  MCUShort::from(name_len as u16).encode(writer);

  //(3) Write raw bytes of the string
  match name_len {
    0 => {}, //Don´t write anything if the tag was nameless
    _ => writer.write_bytes(tag_name.unwrap().as_bytes())
  }
}

#[inline]
fn decode_nbt_tag_name(reader: &mut RawPacketReader) -> Result<Option<String>, Err> {
  //(1) Get length of the tag name
  let name_len: u16 = MCUShort::decode(reader)?.into();

  //(2) Read name if there is one
  match name_len {
    0 => Ok(None),
    name_len => {
      let mut buffer = vec![0x00u8; name_len as usize];
      reader.read_into(&mut buffer);
      Ok(Some(String::from_utf8(buffer)?))
    }
  }
}

impl MCDataType for NbtTag {

  fn decode(reader: &mut RawPacketReader) -> Result<NbtTag, Err> {
    //(1) Read Type byte and tag name
    let nbt_type = reader.read_byte();
    let tag_name = decode_nbt_tag_name(reader)?;

    //(2) Recursively decode the structure
    use NbtTag::*;
    Ok( match nbt_type {
      0 => End,
      1 => Byte(tag_name, MCByte::decode(reader)?.into()),
      2 => Short(tag_name, MCShort::decode(reader)?.into()),
      3 => Int(tag_name, MCInt::decode(reader)?.into()),
      4 => Long(tag_name, MCLong::decode(reader)?.into()),
      5 => Float(tag_name, MCFloat::decode(reader)?.into()),
      6 => Double(tag_name, MCDouble::decode(reader)?.into()),
      7 => {
        //ByteArray requires more of an effort
        let len: i32 = MCInt::decode(reader)?.into();

        if len <= 0 {
          //Empty byte array
          ByteArray(tag_name, Vec::new())
        } else {
          //Byte array with a length
          ByteArray(tag_name, reader.read_bytes(len as usize))
        }
      },
      8 => {
        //String does the same as the byte array
        let len: i16 = MCShort::decode(reader)?.into();

        if len <= 0 {
          //Empty string
          String(tag_name, "".to_string())
        } else {
          //Byte array with a length
          String(tag_name, std::string::String::from_utf8(reader.read_bytes(len as usize))?)
        }
      },
      9 => {
        //List of unnamed NbtTags, starts with type and list length
        let _list_type = reader.read_byte();
        let list_len: i32 = MCInt::decode(reader)?.into();

        if list_len <= 0 {
          //Empty list
          List(tag_name, Vec::new())
        } else {
          //Fill with a list of NBT Tags
          let mut list = Vec::new();
          for _ in 0..list_len {
            list.push(NbtTag::decode(reader)?);
          }
          List(tag_name, list)
        }
      },
      10 => {
        //List of named NbtTags, starts with type and list length
        let list_len: i32 = MCInt::decode(reader)?.into();

        if list_len <= 0 {
          //Empty list
          Compound(tag_name, Vec::new())
        } else {
          //Fill with a list of NBT Tags
          let mut list = Vec::new();
          for _ in 0..list_len {
            list.push(NbtTag::decode(reader)?);
          }
          Compound(tag_name, list)
        }
      },
      11 => {
        //Array of named 32bit big endian ints, array length
        let array_len: i32 = MCInt::decode(reader)?.into();

        if array_len <= 0 {
          //Empty list
          IntArray(tag_name, Vec::new())
        } else {
          //Fill with a list of NBT Tags
          let mut list = Vec::new();
          for _ in 0..array_len {
            list.push(MCInt::decode(reader)?.into());
          }
          IntArray(tag_name, list)
        }
      },
      12 => {
        //Array of named 32bit big endian ints, array length
        let array_len: i32 = MCInt::decode(reader)?.into();

        if array_len <= 0 {
          //Empty list
          LongArray(tag_name, Vec::new())
        } else {
          //Fill with a list of NBT Tags
          let mut list = Vec::new();
          for _ in 0..array_len {
            list.push(MCLong::decode(reader)?.into());
          }
          LongArray(tag_name, list)
        }
      },
      _other => {End} //do nothing for now
    })
  }

  fn encode(&self, writer: &mut RawPacketWriter) {
    use NbtTag::*;

    //(1) Write type byte and tag name
    let nbt_type = writer.write_byte(self.type_code());
    let tag_name = match self {
      End => None,
      Byte(name,_) | Short(name,_) | Int(name,_) | Long(name,_) |
      Float(name,_) | Double(name,_) | ByteArray(name,_) | String(name,_) |
      List(name,_) | Compound(name,_) | IntArray(name,_) | LongArray(name,_)
      => name.clone()
    };
    encode_nbt_tag_name(tag_name, writer);

    //(2) Recursively encode the structure
    match self {
      End => {}, //do nothing,
      Byte(_,byte) => MCByte::from(*byte).encode(writer),
      Short(_, short) => MCShort::from(*short).encode(writer),
      Int(_, int) => MCInt::from(*int).encode(writer),
      Long(_, long) => MCLong::from(*long).encode(writer),
      Float(_, float) => MCFloat::from(*float).encode(writer),
      Double(_, double) => MCDouble::from(*double).encode(writer),
      ByteArray(_, array) => {
        //First we write the length of the array, then the elements
        MCInt::from(array.len() as i32).encode(writer);
        writer.write_bytes(array);
      },
      String(_, string) => {
        //Again, we first encode the length of the string
        MCShort::from(string.len() as i16).encode(writer);
        writer.write_bytes(string.as_bytes());
      },
      List(_, list) => {
        //(1) First we determine the list type, and write it
        let list_type = match list.len() {
          0 => &End,
          _ => list.get(0).unwrap()
        };
        writer.write_byte(list_type.type_code());

        //(2) Write the list length
        MCInt::from(list.len() as i32).encode(writer);

        //(3) Write all elements in the list
        for item in list {
          item.encode(writer);
        }
      },
      Compound(_, compound) => {
        //(1) Write the compound length
        MCInt::from(compound.len() as i32).encode(writer);

        //(2) Write all elements in compound
        for element in compound {
          element.encode(writer);
        }
      },
      IntArray(_,array) => {
        //(1) Write the array length
        MCInt::from(array.len() as i32).encode(writer);

        //(2) Write all elements in array
        for entry in array {
          MCInt::from(*entry).encode(writer);
        }
      },
      LongArray(_, array) => {
        //(1) Again, we first write the array length
        MCInt::from(array.len() as i32).encode(writer);

        //(2) Write all elements in the array
        for entry in array {
          MCLong::from(*entry).encode(writer);
        }
      }
    };
  }

}

impl NbtTag {
  const fn type_code(&self) -> u8 {
    use NbtTag::*;
    match self {
      End             => 0,
      Byte(_,_)       => 1,
      Short(_,_)      => 2,
      Int(_,_)        => 3,
      Long(_,_)       => 4,
      Float(_,_)      => 5,
      Double(_,_)     => 6,
      ByteArray(_,_)  => 7,
      String(_,_)     => 8,
      List(_,_)       => 9,
      Compound(_,_)   => 10,
      IntArray(_,_)   => 11,
      LongArray(_,_)  => 12
    }
  }
}