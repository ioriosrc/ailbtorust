```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use log::Logger;

const LOG = Logger::new(__file__.to_string());

/**
 * Estimates the memory size of a deserialized message object based on the schema definition.
 *
 * The estimation is by no means accurate but may in certain situations (especially when there are
 * no dynamic fields such as arrays or strings) give a better estimation than the number of bytes
 * of the serialized message. For estimating memory size, we assume a V8 JS engine (probably
 * similar for other engines).
 *
 * @param datatypes Map of data types
 * @param typeName Name of the data type
 * @param knownTypeSizes Map of known type sizes (for caching purposes)
 * @returns Estimated object size in bytes
 */
pub fn estimate_message_object_size(
  datatypes: &HashMap<String, MessageDefinition>,
  typeName: &str,
  known_type_sizes: &mut HashMap<&str, usize>,
) -> usize {
  if let Some(&known_size) = known_type_sizes.get(type_name) {
    return known_size;
  }

  if datatypes.is_empty() {
    return OBJECT_BASE_SIZE; // Empty schema -> Empty object.
  }

  let definition = datatypes.get(type_name).expect("Type not found in definitions");
  let size_in_bytes = OBJECT_BASE_SIZE;

  let non_constant_fields = definition.definitions.iter().filter(|field| !field.is_constant);

  if non_constant_fields.count() > MAX_NUM_FAST_PROPERTIES {
    // If there are too many properties, V8 stores Objects in dictionary mode (slow properties)
    // with each object having a self-contained dictionary. This dictionary contains the key, value
    // and details of properties. Below we estimate the size of this additional dictionary. Formula
    // adapted from medium.com/@bpmxmqd/v8-engine-jsobject-structure-analysis-and-memory-optimization-ideas-be30cfcdcd16
    let properties_dict_size = 16 + 5 * 8 + (non_constant_fields.count() as f64).ln().ceil() as usize * 3 * 4;
    size_in_bytes += properties_dict_size;

    // In return, properties are no longer stored in the properties array
    size_in_bytes -= COMPRESSED_POINTER_SIZE * non_constant_fields.count();
  }

  for field in non_constant_fields {
    if field.is_complex {
      let count =
        field.isArray == true
          ? // We are conservative and assume an empty array to avoid memory overestimation.
            (field.array_length.unwrap_or(0))
          : 1;

      if let Some(&known_field_size) = known_type_sizes.get(field.type.as_str()) {
        size_in_bytes += count > 0 { count * known_field_size } else { OBJECT_BASE_SIZE };
        continue;
      }

      if field.checks.contains(field.type.as_str()) {
        // E.g. protobuf allows types to reference itself.
        // For that reason we bail out here to avoid an infinite loop.
        continue;
      }

      let complex_type_object_size = estimate_message_object_size(datatypes, &field.type, known_type_sizes);
      size_in_bytes += count > 0 { count * complex_type_object_size } else { OBJECT_BASE_SIZE };
    } else if field.isArray == true {
      // We are conservative and assume an empty array to avoid memory overestimation.
      // For dynamic messages it is better to use another estimator such as the serialized
      // message size.
      let array_length = field.array_length.unwrap_or(0);
      match field.type.as_str() {
        "int8" | "uint8" => size_in_bytes += TYPED_ARRAY_BASE_SIZE + array_length * 1,
        "int16" | "uint16" => size_in_bytes += TYPED_ARRAY_BASE_SIZE + array_length * 2,
        "int32" | "uint32" | "float32" => size_in_bytes += TYPED_ARRAY_BASE_SIZE + array_length * 4,
        "float64" | "int64" | "uint64" => size_in_bytes += TYPED_ARRAY_BASE_SIZE + array_length * 8,
        _ => {
          let primitive_size = FIELD_SIZE_BY_PRIMITIVE.get(field.type.as_str()).unwrap_or(&5);
          if *primitive_size > 0 {
            size_in_bytes += array_length * primitive_size + OBJECT_BASE_SIZE + COMPRESSED_POINTER_SIZE;
          }
        },
      };
    } else {
      let primitive_size = FIELD_SIZE_BY_PRIMITIVE.get(field.type.as_str()).unwrap_or(&5);
      if *primitive_size > 0 {
        size_in_bytes += primitive_size;
      }
    }
  }

  known_type_sizes.insert(type_name, size_in_bytes);

  return size_in_bytes;
}

/**
 * Determine the size of each schema sub-field. This can be used for estimating
 * the size of sliced messages.
 *
 * @param datatypes
 * @param typeName
 * @param knownTypeSizes
 * @returns
 */
pub fn estimate_message_field_sizes(
  datatypes: &HashMap<String, MessageDefinition>,
  typeName: &str,
  known_type_sizes: &mut HashMap<&str, usize>,
) -> HashMap<String, usize> {
  let mut size_by_field = HashMap::new();
  datatypes.get(type_name).unwrap().definitions.iter().for_each(|field| {
    let field_schema_name = format!("{}-{}", typeName, field.name);
    let field_size_in_bytes = estimate_message_object_size(datatypes, &field_schema_name, known_type_sizes);

    // Subtract the object base size here, it will be added only once per sliced message object.
    size_by_field.insert(field.name.to_string(), field_size_in_bytes - OBJECT_BASE_SIZE);
  });

  return size_by_field;
}

/**
 * Estimate the size in bytes of an arbitrary object or primitive.
 * @param obj Object or primitive to estimate the size for
 * @returns Estimated size in bytes
 */
pub fn estimate_object_size(obj: &dyn std::any::Any) -> usize {
  // catches null and undefined
  if obj.is_none() {
    return SMALL_INTEGER_SIZE;
  }

  let estimate_array_size = |array: &[&dyn std::any::Any]| {
    COMPRESSED_POINTER_SIZE +
      ARRAY_BASE_SIZE +
      array.iter().fold(0, |accumulator, value| accumulator + estimate_object_size(value))
  };

  let estimate_map_size = |map: &HashMap<&str, &dyn std::any::Any>| {
    COMPRESSED_POINTER_SIZE +
      OBJECT_BASE_SIZE +
      map.values().iter().fold(0, |accumulator, value| accumulator + estimate_object_size(value))
  };

  let estimate_set_size = |set: &HashSet<&dyn std::any::Any>| {
    COMPRESSED_POINTER_SIZE +
      OBJECT_BASE_SIZE +
      set.iter().fold(0, |accumulator, value| accumulator + estimate_object_size(value))
  };

  let estimate_object_properties_size = |object: &HashMap<String, &dyn std::any::Any>| -> usize {
    let values_size = object.values().iter().fold(0, |accumulator, value| accumulator + estimate_object_size(value));
    let num_props = object.len();

    if num_props > MAX_NUM_FAST_PROPERTIES {
      // If there are too many properties, V8 stores Objects in dictionary mode (slow properties)
      // with each object having a self-contained dictionary. This dictionary contains the key, value
      // and details of properties. Below we estimate the size of this additional dictionary. Formula
      // adapted from medium.com/@bpmxmqd/v8-engine-jsobject-structure-analysis-and-memory-optimization-ideas-be30cfcdcd16
      let properties_dict_size = 16 + 5 * 8 + (num_props as f64).ln().ceil() as usize * 3 * 4;
      return (
        OBJECT_BASE_SIZE + values_size + properties_dict_size - num_props * COMPRESSED_POINTER_SIZE
      );
    }

    return OBJECT_BASE_SIZE + values_size;
  };

  match obj.downcast_ref::<&str>() {
    Some(&str) => size_in_bytes(COMPRESSED_POINTER_SIZE + OBJECT_BASE_SIZE + str.len() as usize * 4),
    _ => {
      let any_obj = obj.downcast_ref::<i32>().unwrap_or(&0);
      if *any_obj > 0 {
        size_in_bytes(SMALL_INTEGER_SIZE)
      } else if *any_obj == i32::MIN {
        size_in_bytes(i8::MAX as usize + SMALL_INTEGER_SIZE)
      } else if *any_obj == i32::MAX {
        size_in_bytes(i8::MIN as usize + SMALL_INTEGER_SIZE + 1)
      } else if *any_obj > isize::MAX as i32 {
        size_in_bytes(isize::MAX as usize + SMALL_INTEGER_SIZE + 1)
      } else {
        let any_obj = obj.downcast_ref::<f64>().unwrap_or(&0.0);
        if *any_obj.is_nan() || *any_obj == f64::INFINITY || *any_obj == f64::NEG_INFINITY {
          size_in_bytes(HEAP_NUMBER_SIZE)
        } else if *any_obj > 0.0 {
          size_in_bytes(SMALL_INTEGER_SIZE)
        } else if *any_obj == f64::MIN {
          size_in_bytes(i8::MAX as usize + SMALL_INTEGER_SIZE)
        } else if *any_obj == f64::MAX {
          size_in_bytes(i8::MIN as usize + SMALL_INTEGER_SIZE + 1)
        } else if *any_obj > isize::MAX as f64 {
          size_in_bytes(isize::MAX as usize + SMALL_INTEGER_SIZE + 1)
        } else if *any_obj > i32::MAX as f64 {
          size_in_bytes(i32::MAX as usize + SMALL_INTEGER_SIZE + 1)
        } else {
          let any_obj = obj.downcast_ref::<bool>().unwrap_or(&false);
          if *any_obj {
            size_in_bytes(SMALL_INTEGER_SIZE)
          } else {
            size_in_bytes(BIGINT_SIZE)
          }
        }
      }
    },
  };
}
```