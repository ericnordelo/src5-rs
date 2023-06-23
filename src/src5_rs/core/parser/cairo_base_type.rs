// Module for handling extern types in corelib (Base Types)

#[derive(Debug)]
pub struct CairoBaseType {
    pub name: String,
    pub generics: Vec<String>,
}

pub fn get_cairo_base_types() -> [CairoBaseType; 5] {
    [
        CairoBaseType {
            name: "felt252".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "ContractAddress".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "u32".into(),
            generics: Vec::new(),
        },
        CairoBaseType {
            name: "Array".into(),
            generics: vec!["T".into()],
        },
        CairoBaseType {
            name: "u128".into(),
            generics: Vec::new(),
        },
    ]
}

pub fn get_cairo_base_type_from_name(name: &str) -> Option<CairoBaseType> {
    get_cairo_base_types()
        .into_iter()
        .find(|base_type| base_type.name == name)
}
