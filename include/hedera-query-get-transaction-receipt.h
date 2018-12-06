#pragma once

#include "hedera-id.h"
#include "hedera-transaction-id.h"
#include "hedera-query.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint8_t status;
    HederaAccountId* account_id;
    HederaContractId* contract_id;
    HederaFileId* file_id;
} HederaQueryGetTransactionReceiptAnswer;

extern HederaQuery* hedera_query__get_transaction_receipt__new(
    HederaClient*,
    HederaTransactionId transaction_id
);

extern HederaError hedera_query__get_transaction_receipt__get(HederaQuery*, HederaQueryGetTransactionReceiptAnswer*);

#ifdef __cplusplus
}
#endif
