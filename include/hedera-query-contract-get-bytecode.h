#pragma once

#include "hedera-id.h"
#include "hedera-transaction-id.h"
#include "hedera-query.h"

#ifdef __cplusplus
extern "C" {
#endif

extern HederaQuery* hedera_query__contract_get_bytecode__new(
    HederaClient*,
    HederaContractID contract_id
);

extern HederaError hedera_query__contract_get_bytecode__get(HederaQuery*, const uint_8*);

#ifdef __cplusplus
}
#endif
