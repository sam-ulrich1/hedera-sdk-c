use crate::timestamp::CTimestamp;
use hedera::transaction::TransactionAdminContractDelete;
use hedera::transaction::TransactionAdminFileDelete;
use hedera::ContractId;
use hedera::FileId;

def_tx_new!(TransactionAdminFileDelete: hedera_transaction__admin_file_delete__new(FileId));

def_tx_method!(
    TransactionAdminFileDelete: hedera_transaction__admin_file_delete__set_expire_at(CTimestamp):
        expire_at
);

def_tx_new!(
    TransactionAdminContractDelete: hedera_transaction__admin_contract_delete__new(ContractId)
);

def_tx_method!(
    TransactionAdminContractDelete:
        hedera_transaction__admin_contract_delete__set_expire_at(CTimestamp): expire_at
);
