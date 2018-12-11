use hedera::transaction::TransactionFileAppend;
use hedera::FileId;

def_tx_new!(TransactionFileAppend: hedera_transaction__file_append__new(FileId, Vec<u8>));
