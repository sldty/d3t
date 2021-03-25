pub enum Definition {
    /// For self-referential definitions.
    Recursive,
    /// A bit, either 0 or 1.
    Bit,
    /// A fixed-size collection of data.
    /// For example, a byte is a `Chunk { Bit, 8 }`.
    Chunk {
        item: Box<Definition>,
        size: u64,
    },
    /// A growable collection of data.
    /// For example, an arbitrary unsigned number could be a
    /// `List { Bit }`
    List { item: Box<Definition> },
    /// A link to another definition.
    /// Defined as the sha3 of the serialized representation of that definition.
    /// TODO: uniqueness
    Link { sha3: [u8; 32] },
    /// A product type, i.e. a struct.
    /// Note that there are no identifiers
    /// fields are accessed by index
    Product { fields: Vec<Definition> },
    /// A sum type, i.e. an enum.
    /// Note that there are no identifiers
    /// variants are tagged by index
    Sum { variants: Vec<Definition> },
}

impl Definition {
    pub fn link(d3_type: D3Type) -> Definition {

    }
}
