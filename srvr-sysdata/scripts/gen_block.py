#  Copyright (C) 2022 Raúl Wolters
#  
#  This file is part of srvr.
#  
#  srvr is free software: you can redistribute it and/or modify it under the
#  terms of the European Union Public License (EUPL), provided that you publish
#  your modifications under the terms of the EUPL or another compatible license
#  as specified by the EUPL v1.2 or higher.
#
#  As the copyright holder is a citizen of the Kingdom of the Netherlands, this
#  license agreement shall be governed by dutch law, as specified in clause 15
#  of the EUPL v1.2.
#
#  srvr is distributed in the hope that it will be useful, but WITHOUT ANY
#  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
#  A PARTICULAR PURPOSE.  See the European Union Public License for more details.
#  
#  You should have received a copy of the European Union Public License in a
#  official language of the European Union along with srvr. If not, see
#  <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12> for the full
#  text of the license in any official language of the European Union.

import json
import re

from numpy import block

#Global vars
BLOCK_DATA = "../data/blocks.json"
TARGET_FILE = "../src/generated/block.rs"
PROPERTY_TRAIT = "BlockProperty"
BLOCKSTATE_TRAIT = "BlockState"

COPYRIGHT = """/*
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
*/"""

AUTO_GENERATED = """/*
-------------THIS IS AN AUTO-GENERATED FILE. DO NOT EDIT IT MANUALLY------------
*/"""

def main():
  
  properties = {}
  block_states = {}
  
  with open(BLOCK_DATA) as file:
    raw_js = json.loads(file.read())
    
    #First we get all possible unique block properties
    for block_name, fields in raw_js.items():
      if 'properties' in fields:
        for property_name, values in fields['properties'].items():
          if property_name not in properties:
            properties[property_name] = values
          else:
            for value in values:
              if value not in properties[property_name]:
                properties[property_name].append(value)
        
    block_states = raw_js
  
  with open(TARGET_FILE, 'w+') as f:
    #Add copyright notice and imports
    f.write(f"{COPYRIGHT}\n\n{AUTO_GENERATED}\n\n")
    f.write("use crate::block::{BlockProperty, BlockState};\n\n")
    
    #Next we generate an enum for each property, which we must then implement
    for property_name, values in properties.items():   
      if next(iter(values)).isdigit():
        enum = generate_numeric_property(property_name)
      else:
        enum = generate_property_enum(property_name, values)     
      f.write(enum)      
      print(enum)
      
    #Now we generate a struct for each block state (which sucks ass)
    print(block_states)
    for block_state, dict in block_states.items():
      parsed_name = parse_name(block_state)
      printout = None
      
      if 'properties' not in dict:
        #This is a simple block state (one state for one block)
        states = dict['states']
        id = int(states[0]['id'])
        printout = generate_simple_block_state(block_state, parsed_name, id)
        f.write(printout)
        print(printout)
        continue
      
      #We have a more complicated case, let's set some flags
      props = dict['properties']
      singular = len(props) == 1
      numeric = next(iter(props.values()))[0].isdigit()
      
      if singular and not numeric:
        #This block has only one property so can be a simple tuple of an enum
        printout = generate_tuple_block_state(block_state, parsed_name, props, dict['states'])
      elif singular and numeric:
        #This block has one numeric property   
        printout = generate_numeric_block_state(block_state, parsed_name, props, dict['states'])
      else:
        printout = generate_complex_struct(block_state, parsed_name, props, dict['states'])
        print(printout)

      if printout is not None: f.write(printout)
      print(printout)
     
    #End with another disclaimer
    f.write(AUTO_GENERATED)
  
  pass

def generate_property_enum(property_name, property_values) -> str:
  enum = "#[derive(Debug, Clone, PartialEq, Eq)]\n"
  enum += "#[repr(u8)]\n"
  enum += f"pub enum {parse_name(property_name)} {{\n"
  for value in property_values:
    enum += f"  {parse_name(value)},\n"
  enum = enum[:-2] #remove last comma
  enum += "\n}\n"
  enum += f"impl {PROPERTY_TRAIT} for {parse_name(property_name)} {{}}\n\n"
  return enum

def generate_numeric_property(property_name) -> str:
  struct = "#[derive(Debug, Clone, PartialEq, Eq)]\n";
  struct += f"pub struct {parse_name(property_name)}(u8);\n"
  struct += f"impl {PROPERTY_TRAIT} for {parse_name(property_name)} {{}}\n\n"
  return struct

def generate_simple_block_state(unparsed_name: str, block_name: str, id: int) -> str:
  struct = "#[derive(Debug, Clone, PartialEq, Eq)]\n"
  struct += f"pub struct {block_name};\n"
  struct += f"impl {BLOCKSTATE_TRAIT} for {block_name} {{\n"
  struct += f"  fn get_id(&self) -> u16 {{{id}}}\n"
  struct += f"  fn get_name(&self) -> &'static str {{\"{unparsed_name}\"}}\n"
  struct += "}\n\n"
  return struct

def generate_numeric_block_state(unparsed_name: str, block_name: str, properties, values) -> str:
  inner = parse_name(next(iter(properties)))
  
  struct = "#[derive(Debug, Clone, PartialEq, Eq)]\n"
  struct += f"pub struct {block_name}({inner});\n"
  struct += f"impl {BLOCKSTATE_TRAIT} for {block_name} {{\n"
  struct += f"  fn get_id(&self) -> u16 {{ match self.0.0 {{\n"
  for config in values:
    id = int(config['id'])
    var = config['properties'][next(iter(properties))]
    struct += f"   {var} => {id},\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n"
  struct += f"  fn get_name(&self) -> &'static str {{ match self.0.0 {{\n"
  for config in values:
    var = config['properties'][next(iter(properties))]
    struct += f"    {var} => \"{unparsed_name}::{var}\",\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n}\n\n"
  return struct

def generate_tuple_block_state(unparsed_name: str, block_name: str, properties, values) -> str:
  enum = parse_name(next(iter(properties)))
  
  struct = "#[derive(Debug, Clone, PartialEq, Eq)]\n"
  struct += f"pub struct {block_name}({enum});\n"
  struct += f"impl {BLOCKSTATE_TRAIT} for {block_name} {{\n"
  struct += f"  fn get_id(&self) -> u16 {{ match self.0 {{\n"
  for config in values:
    id = int(config['id'])
    var = config['properties'][next(iter(properties))]
    struct += f"    {enum}::{parse_name(var)} => {id},\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n"
  struct += f"  fn get_name(&self) -> &'static str {{ match self.0 {{\n"
  for config in values:
    var = config['properties'][next(iter(properties))]
    struct += f"    {enum}::{parse_name(var)} => \"{unparsed_name}::{var}\",\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n}\n\n"
  return struct

def generate_complex_struct(unparsed_name: str, block_name: str, properties, values) -> str:
  #First we need to get all the properties of the block state
  tuples = []
  for prop in iter(properties):
    tuples.append(parse_name(prop))
  struct = "#[derive(Debug, Clone, PartialEq, Eq)]\n"
  struct += f"pub struct {block_name}("
  for tup in tuples: struct += f"{tup}, "
  struct = struct[:-2]
  struct += f");\nimpl {BLOCKSTATE_TRAIT} for {block_name} {{\n"
  struct += f"  fn get_id(&self) -> u16 {{ match self {{\n"
  for config in values:
    id = int(config['id'])
    struct += f"    {block_name}("
    for property, value in config['properties'].items():
      if next(iter(value)).isdigit():
        #numeric property!
        struct += f"{parse_name(property)}({value}), "
      else:
        struct += f"{parse_name(property)}::{parse_name(value)}, "
    struct = struct[:-1]
    struct += f") => {id},\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n"
  struct += f"  fn get_name(&self) -> &'static str {{ match self {{\n"
  for config in values:
    struct += f"    {block_name}("
    for property, value in config['properties'].items():
      if next(iter(value)).isdigit():
        #numeric property!
        struct += f"{parse_name(property)}({value}), "
      else:
        struct += f"{parse_name(property)}::{parse_name(value)}, "
    struct = struct[:-1]
    struct += f") => \"{unparsed_name}::{config['properties']}\",\n"
  struct += "    _ => panic!(\"Invalid block state\")\n  }}\n}\n\n"    
  return struct
  
def parse_name(name: str) -> str:
  
  #(1) we split the name at invalid chars
  split = re.split('_|:|-', name)
  
  #(2) capitalise each split name
  split = [part.capitalize() for part in split]
  
  #(3) add them all together
  ret = ''
  for part in split: ret += part
  return ret

if __name__ == "__main__":
  main()