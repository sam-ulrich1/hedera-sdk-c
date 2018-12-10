use hedera::transaction::TransactionFileDelete;
use hedera::FileId;

def_tx_new!(TransactionFileDelete: hedera_transaction__file_delete__new(FileId));
