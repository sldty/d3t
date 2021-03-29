use std::collections::HashMap;

/// Compact serializable representation of some data of a type
pub enum Data {
    // Atoms

    Double(f64),
    Word(u64),
    Byte(u8),
    Bool(bool),
    String(String),

    // Compounds

    Chunk(Vec<Data>),
    List(Vec<Data>),
    Product(Vec<Data>),
    Sum(u8, Box<Data>),
    Map(HashMap<Data, Data>),
    Link(Box<Data>),
}

// given some data, serialize it
// given some serialized data and a typedef, deserialize it
