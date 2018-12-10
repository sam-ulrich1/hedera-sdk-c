use hedera::transaction::TransactionAdminFileRecover;
use hedera::transaction::TransactionAdminContractRecover;
use hedera::FileId;
use hedera::ContractId;

def_tx_new!(TransactionAdminFileDelete: hedera_transaction__admin_file_delete__new(FileId));

//todo: expire at

def_tx_new!(TransactionAdminContractDelete: hedera_transaction__admin_contract_delete__new(ContractId));

//todo: expire at
