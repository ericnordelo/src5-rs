// Module for handling extern types in corelib (Base Types)

#[derive(Debug)]
pub struct CairoBaseType {
    pub name: String,
    pub generics: Vec<String>,
}

pub fn get_cairo_base_types() -> [CairoBaseType; 3] {
    [
        CairoBaseType {
            name: "felt252".into(),
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
