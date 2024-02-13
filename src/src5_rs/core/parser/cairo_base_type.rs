// Module for handling extern types in corelib (Base Types)

#[derive(Debug)]
pub struct CairoBaseType {
    pub name: String,
    pub generics: Vec<String>,
}

pub fn get_cairo_base_types() -> [CairoBaseType; 40] {
    [
        CairoBaseType {
            name: "bytes31".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "felt252".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "usize".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Array".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "Nullable".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "Box".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "Felt252Dict".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "SquashedFelt252Dict".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "Felt252DictEntry".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "EcOp".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "EcPoint".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "NonZeroEcPoint".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "EcState".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "GasBuiltin".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "BuiltinCosts".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Pedersen".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Poseidon".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "System".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "SyscallResult".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "NonZero".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "u8".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "u16".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "u32".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "u64".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "u128".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "i8".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "i16".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "i32".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "i64".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "i128".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "U128MulGuarantee".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Bitwise".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "RangeCheck".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "SegmentArena".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "ClassHash".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Secp256k1Point".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Secp256r1Point".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "ContractAddress".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "StorageAddress".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "StorageBaseAddress".into(),
            generics: Vec::new(),
        },
    ]
}

pub fn get_cairo_base_type_from_name(name: &str) -> Option<CairoBaseType> {
    get_cairo_base_types()
        .into_iter()
        .find(|base_type| base_type.name == name)
}
