use aptos_state_view::StateView;
use aptos_types::{
    transaction::{Transaction, TransactionOutput},
    vm_status::VMStatus
};
use aptos_vm::VMExecutor;

pub struct ZkSyncWrapperVM;

impl VMExecutor for ZkSyncWrapperVM {
    /// Execute a block of `transactions`. The output vector will have the exact same length as the
    /// input vector. The discarded transactions will be marked as `TransactionStatus::Discard` and
    /// have an empty `WriteSet`. Also `state_view` is immutable, and does not have interior
    /// mutability. Writes to be applied to the data view are encoded in the write set part of a
    /// transaction output.
    fn execute_block(
        transactions: Vec<Transaction>,
        state_view: &impl StateView,
    ) -> Result<Vec<TransactionOutput>, VMStatus> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: i32 = 2 + 2;
        assert_eq!(result, 4);
    }
}
