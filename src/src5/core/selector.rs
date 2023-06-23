use cairo_lang_starknet::contract::starknet_keccak;
use num_bigint::BigUint;

pub fn get_selector_from_signature(signature: &str) -> BigUint {
    starknet_keccak(signature.as_bytes())
}
