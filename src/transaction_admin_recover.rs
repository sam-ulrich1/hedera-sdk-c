use hedera::transaction::TransactionAdminFileRecover;
use hedera::transaction::TransactionAdminContractRecover;
use hedera::FileId;
use hedera::ContractId;

def_tx_new!(TransactionAdminFileRecover: hedera_transaction__admin_file_recover__new(FileId));

def_tx_new!(TransactionAdminContractRecover: hedera_transaction__admin_contract_recover__new(ContractId));
