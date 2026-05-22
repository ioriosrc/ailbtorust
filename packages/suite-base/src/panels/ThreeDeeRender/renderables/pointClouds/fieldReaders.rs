```rust
use std::ptr;

struct DataView {
    data: *const u8,
    len: usize,
}

pub struct PointField {
    offset: usize,
    count: usize,
    datatype: String, // Assuming string representation of the datatype for simplicity
}

fn int8_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> i8 {
    move |view, point_offset| {
        let value = view.get_i8(point_offset + field_offset);
        if normalize {
            (value as f64 / 0x7f).clamp(-1.0, 1.0) as i8
        } else {
            value
        }
    }
}

fn uint8_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> u8 {
    move |view, point_offset| {
        let value = view.get_u8(point_offset + field_offset);
        if normalize {
            value as f64 / 0xff
        } else {
            value
        }
    }
}

fn int16_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> i16 {
    move |view, point_offset| {
        let value = view.get_i16(point_offset + field_offset);
        if normalize {
            (value as f64 / 0x7fff).clamp(-1.0, 1.0) as i16
        } else {
            value
        }
    }
}

fn uint16_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> u16 {
    move |view, point_offset| {
        let value = view.get_u16(point_offset + field_offset);
        if normalize {
            value as f64 / 0xffff
        } else {
            value
        }
    }
}

fn int32_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> i32 {
    move |view, point_offset| {
        let value = view.get_i32(point_offset + field_offset);
        if normalize {
            (value as f64 / 0x7fffffff).clamp(-1.0, 1.0) as i32
        } else {
            value
        }
    }
}

fn uint32_reader(field_offset: usize, normalize: bool) -> fn(&DataView, usize) -> u32 {
    move |view, point_offset| {
        let value = view.get_u32(point_offset + field_offset);
        if normalize {
            value as f64 / 0xffffffff
        } else {
            value
        }
    }
}

fn float32_reader(field_offset: usize) -> fn(&DataView, usize) -> f32 {
    move |view, point_offset| view.get_f32(point_offset + field_offset)
}

fn float64_reader(field_offset: usize) -> fn(&DataView, usize) -> f64 {
    move |view, point_offset| view.get_f64(point_offset + field_offset)
}

pub struct FieldReader {
    pub func: fn(&DataView, usize) -> i8,
}

impl FieldReader {
    pub fn from_int8(field_offset: usize, normalize: bool) -> Self {
        Self { func: int8_reader(field_offset, normalize) }
    }

    pub fn from_uint8(field_offset: usize, normalize: bool) -> Self {
        Self { func: uint8_reader(field_offset, normalize) }
    }

    pub fn from_int16(field_offset: usize, normalize: bool) -> Self {
        Self { func: int16_reader(field_offset, normalize) }
    }

    pub fn from_uint16(field_offset: usize, normalize: bool) -> Self {
        Self { func: uint16_reader(field_offset, normalize) }
    }

    pub fn from_int32(field_offset: usize, normalize: bool) -> Self {
        Self { func: int32_reader(field_offset, normalize) }
    }

    pub fn from_uint32(field_offset: usize, normalize: bool) -> Self {
        Self { func: uint32_reader(field_offset, normalize) }
    }

    pub fn from_float32(field_offset: usize) -> Self {
        Self { func: float32_reader(field_offset) }
    }

    pub fn from_float64(field_offset: usize) -> Self {
        Self { func: float64_reader(field_offset) }
    }
}

pub fn is_supported_field(field: &PointField) -> bool {
    // Only PointFields with count === 1 are supported (doesn't apply to PackedElementFields)
    field.count == 1
}

pub fn get_reader(
    field: &PointField,
    stride: usize,
    normalize: bool,
    force_type: Option<PointFieldType>,
) -> FieldReader {
    if !is_supported_field(field) {
        return FieldReader { func: None };
    }

    let numeric_type = field.datatype.parse::<i8>().unwrap_or_default();
    if numeric_type == 0 {
        match force_type {
            Some(PointFieldType::INT8) => FieldReader { func: int8_reader(field.offset, normalize) },
            _ => FieldReader { func: None },
        }
    } else {
        match force_type {
            Some(numeric_type) => match numeric_type as i8 {
                0 => FieldReader { func: int8_reader(field.offset, normalize) },
                1 => FieldReader { func: uint8_reader(field.offset, normalize) },
                2 => FieldReader { func: int16_reader(field.offset, normalize) },
                3 => FieldReader { func: uint16_reader(field.offset, normalize) },
                4 => FieldReader { func: int32_reader(field.offset, normalize) },
                5 => FieldReader { func: uint32_reader(field.offset, normalize) },
                6 => FieldReader { func: float32_reader(field.offset) },
                7 => FieldReader { func: float64_reader(field.offset) },
                _ => FieldReader { func: None },
            },
            _ => FieldReader { func: None },
        }
    }
}
```

### Explanation:
1. **DataView Struct**: A simple struct to represent the view into a data buffer.
2. **PointField Struct**: A struct to represent the point field with offset, count, and datatype.
3. **int8_reader, uint8_reader, int16_reader, uint16_reader, int32_reader, uint32_reader, float32_reader, float64_reader**: Functions to read integer and floating-point values from the data view.
4. **FieldReader Struct**: A struct to represent a reader function that can be used to read data from the data view.
5. **is_supported_field**: Function to check if the given field is supported.
6. **get_reader**: Function to get the appropriate reader based on the field's datatype and optional force type.

This Rust code should perform similarly to the original TypeScript/React function, providing a simple interface for reading data from a PointField or PackedElementField.