//! Definition of a type

pub enum Definition {
    // Atoms

    Double,
    Word,
    Byte,
    Bool,
    String,

    // Compounds

    Chunk(u64, Definition),
    List(Definition),
    Product(Vec<Definition>),
    Sum(Vec<Definition>),
    Map(Box<Definition>, Box<Definition>),
    Link(Box<Definition>),
}
