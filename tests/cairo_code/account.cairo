use array::ArrayTrait;
use array::SpanTrait;
use starknet::ContractAddress;

struct Call {
    to: ContractAddress,
    selector: felt252,
    calldata: Array<felt252>
}

struct Call2<T> {
    to: Span<T>,
}

enum Call3<T> {
    faa: bool,
    fee: Span<T>,
    foo: T,
}

trait ISRC5 {
    fn supports_interface(interface_id: felt252) -> bool;
}

trait IAccount {
    fn __execute__(calls: Array<Call>) -> Array<Span<felt252>>;
    fn __validate__(calls: Array<Call>) -> felt252;
    fn __validate_declare__(class_hash: felt252) -> felt252;
    fn is_valid_signature(message: felt252, signature: Array<felt252>) -> u32;
    fn supports_interface(interface_id: u32) -> bool;
}

trait IAccount2 {
    fn __execute__(calls: Array<Call>) -> Array<Span<felt252>>;
    fn __validate__(calls: Array<Call>) -> felt252;
    fn __validate_declare__(class_hash: felt252) -> felt252;
    fn is_valid_signature(message: felt252, signature: Array<felt252>) -> u32;
    fn supports_interface(interface_id: u32) -> bool;
}
