use starknet::ContractAddress;
use starknet::account::Call;

trait IAccount {
    fn __execute__(calls: Array<Call>) -> Array<Span<Result<Option<Call>,System>>>;
}
