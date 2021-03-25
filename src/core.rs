const DEFINITION_DEFINITION: Definition = Definition::Sum {
    variants: vec![
        // Bit
        Definition::Product { fields: vec![] },
        // Chunk
        Definition::Product {
            fields: vec![
                Definition::Recursive,
                Definition::link(NUMBER_DEFINITION),
            ],
        },
        // List
        Definition::Recursive,
        // Link
        Definition::link(SHA3_DEFINITION),
        // Product
        Definition::List {
            item: Box::new(Definition::Recursive),
        },
        // Sum
        Definition::List {
            item: Box::new(Definition::Recursive),
        },
    ]
};

const BYTE_DEFINITION: Definition = Definition::Chunk {
    item: Box::new(Definition::Bit),
    size: 8,
};

const DATA_DEFINITION: Definition = Definition::Sum {
    variants: vec![
        // Bytes
        Definition::List { item: Box::new(Definition::link(BYTE_DEFINITION)) },
        // Chunk
        Definition::List { item: Box::new(Definition::Recursive) },
        // List
        Definition::List { item: Box::new(Definition::Recursive) },
        // Link
        Definition::link(D3_TYPE_DEFINITION),
        // Product
        Definition::List { item: Box::new(Definition::Recursive) },
        // Sum
        Definition::Product {
            fields: vec![

            ]
        },
    ]
};

const D3_TYPE_DEFINITION: Definition = Definition::Product {
    fields: vec![
        Definition::link(KEY_DEFINITION),
        Definition::link(SIGNATURE_DEFINITION),
        Definition::link(DEFINITION_DEFINITION),
        Definition::link(DATA_DEFINITION),
    ]
};
