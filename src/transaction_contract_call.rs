use hedera::transaction::TransactionContractCall;
use hedera::ContractId;

def_tx_new!(TransactionContractCall: hedera_transaction__contract_call__new(ContractId));

def_tx_method!(TransactionContractCall: hedera_transaction__contract_call__set_gas(i64): gas);

def_tx_method!(TransactionContractCall: hedera_transaction__contract_call__set_amount(i64): amount);

def_tx_method!(
    TransactionContractCall: hedera_transaction__contract_call__set_function_parameters(array_of(u8)):
        function_parameters
);
