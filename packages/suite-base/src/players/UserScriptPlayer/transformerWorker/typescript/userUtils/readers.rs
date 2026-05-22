```rust
struct Uint8Reader {
    buffer: [u8; 1];
}

impl Uint8Reader {
    fn new(offset: usize) -> Self {
        Self { buffer: [0; 1] }
    }

    fn read(&self, data: &[u8]) -> u8 {
        if data.len() < offset + 1 {
            panic!("Not enough data");
        }
        self.buffer[0] = data[offset];
        self.buffer[0]
    }
}

struct Uint16Reader {
    buffer: [u8; 2];
}

impl Uint16Reader {
    fn new(offset: usize) -> Self {
        Self { buffer: [0; 2] }
    }

    fn read(&self, data: &[u8]) -> u16 {
        if data.len() < offset + 2 {
            panic!("Not enough data");
        }
        let mut value = 0;
        for i in (0..2).rev() {
            value |= (data[offset + i] as u16) << (i * 8);
        }
        value
    }
}

struct Int8Reader {
    buffer: [u8; 1];
}

impl Int8Reader {
    fn new(offset: usize) -> Self {
        Self { buffer: [0; 1] }
    }

    fn read(&self, data: &[u8]) -> i8 {
        if data.len() < offset + 1 {
            panic!("Not enough data");
        }
        self.buffer[0] as i8
    }
}

struct Int16Reader {
    buffer: [u8; 2];
}

impl Int16Reader {
    fn new(offset: usize) -> Self {
        Self { buffer: [0; 2] }
    }

    fn read(&self, data: &[u8]) -> i16 {
        if data.len() < offset + 2 {
            panic!("Not enough data");
        }
        let mut value = 0;
        for i in (0..2).rev() {
            value |= (data[offset + i] as u16) << (i * 8);
        }
        value as i16
    }
}

struct Float32Reader {
    buffer: [u8; 4];
}

impl Float32Reader {
    fn new(offset: usize) -> Self {
        Self { buffer: [0; 4] }
    }

    fn read(&self, data: &[u8]) -> f32 {
        if data.len() < offset + 4 {
            panic!("Not enough data");
        }
        let mut value = 0.0;
        for i in (0..4).rev() {
            value |= ((data[offset + i] as u32) & 0xFF) << (i * 8);
        }
        unsafe { std::mem::transmute::<f32, f32>(value) }
    }
}

fn get_reader(datatype: usize, offset: usize) -> Box<dyn Fn(&[u8]) -> i64> {
    match datatype {
        DATATYPE.float32 => Box::new(|data| Float32Reader { buffer: [0; 4] }.read(data)),
        DATATYPE.uint8 => Box::new(|data| Uint8Reader { buffer: [0; 1] }.read(data)),
        DATATYPE.uint16 => Box::new(|data| Uint16Reader { buffer: [0; 2] }.read(data)),
        DATATYPE.int16 => Box::new(|data| Int16Reader { buffer: [0; 2] }.read(data)),
        DATATYPE.int32 => Box::new(|data| Int32Reader { buffer: [0; 4] }.read(data)),
        _ => panic!("Unsupported datatype: '{:?}'", datatype),
    }
}
```