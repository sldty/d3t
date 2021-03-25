/// Whenever lengths are serialized, we serialize them as
/// arbitrary numbers.
/// i.e. a series of bytes, where first bit is the continue bit
/// and the rest are the data bits.
pub enum Data {
    /// Whenever we have a series of fixed-sized values,
    /// We chunk them together into a bytearray.
    /// So a `Chunk { Bit, 8 }` is not 8 seperate Data,
    /// it's a `Bytes { bytes: [0xFF] }`
    /// Serialized as simply each byte.
    Bytes { bytes: Vec<u8> },
    /// serialized as length, then each item in chunk
    Chunk { items: Vec<Data> },
    /// Serialized as length, then each item in the chunk for it's size
    List { items: Vec<Data> },
    /// Serialized as length, then as D3Type
    Link { d3t: Box<D3Type> },
    /// Each item is serialized in order
    Product { fields: Vec<Data> },
    /// Serialized as length, then variant, then item.
    Sum {
        variant: u64,
        item:    Box<Data>,
    },
}

impl Data {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self {
            Data::Bytes { bytes } => bytes.clone(),
            Data::Chunk { items } => {
                let mut bytes = vec![];
                for item in items { bytes.append(&mut Data::to_bytes(item)); }
                bytes
            },
            Data::List { items } => {
                let mut bytes = vec![];
                for item in items { bytes.append(&mut Data::to_bytes(item)); }
                bytes
            },
            Data::Link { d3t } => Data::to_bytes(D3Type::to_data(d3t)),
            Data::Product { fields } => {
                let mut bytes = vec![];
                for field in fields { bytes.append(&mut Data::to_bytes(field)); }
                bytes
            },
            Data::Sum { variant, item } => {
                let mut bytes = number_to_bytes(variant);
                bytes.append(&mut Data::to_bytes(item));
                bytes
            },
        };

        let mut total = number_to_bytes(bytes.len());
        total.push(bytes);
        return total;
    }
}
