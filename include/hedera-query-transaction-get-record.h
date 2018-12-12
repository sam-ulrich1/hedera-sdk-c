typedef struct {
    // fixme: use enum here
    uint8_t status;
    HederaAccountId* account_id;
    HederaContractId* contract_id;
    HederaFileId* file_id;
} HederaTransactionRecord;
