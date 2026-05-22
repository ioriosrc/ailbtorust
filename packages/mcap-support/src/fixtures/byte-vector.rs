```rust
use flatbuffers::{FlatBufferBuilder, Offset, Vector};

pub struct ByteVector<'a> {
    bb: Option<&'a [u8]>,
    offset: Offset,
}

impl ByteVector<'_> {
    pub fn __init(&mut self) {
        self.offset = self.bb.as_ref().map_or(0, |bb| bb.len());
    }

    pub fn get_root_as_byte_vector(bb: &Self) -> Self {
        Self { bb: Some(bb.bb), offset: bb.offset }
    }

    pub fn size_prefixed_get_root_as_byte_vector(bb: &Self) -> Self {
        let offset = bb.offset + 4;
        Self { bb: Some(bb.bb), offset }
    }

    pub fn data(&self, index: usize) -> Option<u8> {
        if let Some(bytes) = self.bb.as_ref() {
            bytes.get(self.offset + index)
        } else {
            None
        }
    }

    pub fn data_length(&self) -> usize {
        if let Some(bytes) = self.bb.as_ref() {
            bytes.len() - self.offset
        } else {
            0
        }
    }

    pub fn data_array(&self) -> Option<Vec<u8>> {
        if let Some(bytes) = self.bb.as_ref() {
            Some(bytes.to_vec()[self.offset..])
        } else {
            None
        }
    }

    pub fn start_byte_vector(builder: FlatBufferBuilder<'_>) {
        builder.start_object(1);
    }

    pub fn add_data(builder: &mut FlatBufferBuilder, data_offset: Offset) {
        builder.add_field_offset(0, data_offset, 0);
    }

    pub fn create_data_vector(
        builder: &mut FlatBufferBuilder,
        data: &[u8],
    ) -> Offset {
        let vector_offset = builder.start_vector(1, data.len(), 1);
        for byte in data {
            builder.add_int8(*byte);
        }
        builder.end_vector(vector_offset)
    }

    pub fn start_data_vector(builder: &mut FlatBufferBuilder, num_elems: usize) {
        builder.start_vector(1, num_elems, 1);
    }

    pub fn end_byte_vector(builder: &mut FlatBufferBuilder) -> Offset {
        builder.end_object()
    }

    pub fn finish_byte_vector_buffer(builder: &mut FlatBufferBuilder, offset: Offset) {
        builder.finish(offset);
    }

    pub fn finish_size_prefixed_byte_vector_buffer(
        builder: &mut FlatBufferBuilder,
        offset: Offset,
    ) {
        builder.finish(offset, None, true);
    }

    pub fn create_byte_vector(builder: &mut FlatBufferBuilder, data_offset: Offset) -> Offset {
        ByteVector::start_byte_vector(builder);
        ByteVector::add_data(builder, data_offset);
        ByteVector::end_byte_vector(builder)
    }
}
```